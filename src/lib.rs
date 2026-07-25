//! Core library for MSI `CoolerBoost`.
//!
//! Provides shared functionality for checking, toggling and displaying the
//! `CoolerBoost` fan-boost state on MSI laptops through the `isw` tool. Also
//! includes helpers for parsing and updating the Hyprland keybinding as well
//! as generating a status icon and desktop notifications.

pub mod toggle_mode;
pub mod tray_mode;

use dirs::config_dir;
use notify_rust::Notification;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Path to the state file used to mirror the current `CoolerBoost` state.
///
/// The file is created when `CoolerBoost` is ON and removed when OFF. It lives
/// on tmpfs so it is automatically cleared on reboot.
pub const STATE_FILE: &str = "/tmp/isw_coolerboost";

/// Relative path to the Hyprland bindings file inside the user's config directory.
pub const BINDINGS_FILE: &str = "hypr/bindings.conf";

/// Returns the path to the Hyprland bindings file.
///
/// Combines the user's XDG config directory with [`BINDINGS_FILE`].
#[must_use]
fn bindings_path() -> Option<PathBuf> {
    config_dir().map(|dir| dir.join(BINDINGS_FILE))
}

/// Checks whether `CoolerBoost` is currently enabled.
///
/// This is determined by the presence of [`STATE_FILE`].
///
/// # Returns
/// `true` if the state file exists, otherwise `false`.
#[must_use]
pub fn check_status() -> bool {
    PathBuf::from(STATE_FILE).exists()
}

/// Retrieves the currently configured `CoolerBoost` shortcut from Hyprland bindings.
///
/// Parses `~/.config/hypr/bindings.conf` looking for a comment line containing
/// `CoolerBoost`, then the next non-empty, non-comment `bindd = ...` line, and
/// returns a formatted string such as `"SUPER + F10"`.
///
/// # Returns
/// The parsed shortcut, or `"Unknown"` if parsing fails.
#[must_use]
pub fn get_current_shortcut() -> String {
    let Some(content) = bindings_path().and_then(|path| fs::read_to_string(path).ok()) else {
        return "Unknown".to_string();
    };

    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if line.contains("CoolerBoost") {
            while let Some(next) = lines.peek() {
                let trimmed = next.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    lines.next();
                    continue;
                }
                let Some(bind) = next.strip_prefix("bindd = ") else {
                    break;
                };
                let parts: Vec<&str> = bind.split(',').collect();
                if parts.len() >= 2 {
                    let modifiers = parts[0].trim();
                    let key = parts[1].trim();
                    if !modifiers.is_empty() && !key.is_empty() {
                        return format!("{modifiers} + {key}");
                    }
                }
                break;
            }
        }
    }
    "Unknown".to_string()
}

/// Updates the `CoolerBoost` keybinding in the Hyprland config.
///
/// Replaces the existing `bindd` line associated with the `CoolerBoost`
/// comment block in `~/.config/hypr/bindings.conf` and triggers a Hyprland
/// reload.
///
/// # Arguments
/// * `modifiers` - Modifier keys (e.g. `"SUPER"`).
/// * `key` - Key to bind (e.g. `"F10"`). It will be uppercased automatically.
///
/// # Errors
/// Returns an error if the config directory or bindings file cannot be
/// determined/read/written, or if a `CoolerBoost` binding block is missing.
pub fn set_shortcut(modifiers: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = bindings_path().ok_or("unable to determine config directory")?;
    let content = fs::read_to_string(&path)?;

    let new_line = format!(
        "bindd = {modifiers}, {}, Toggle CoolerBoost, exec, msi-coolerboost toggle",
        key.to_uppercase()
    );

    let mut new_content = String::with_capacity(content.len() + new_line.len());
    let mut lines = content.lines().peekable();
    let mut replaced = false;

    while let Some(line) = lines.next() {
        new_content.push_str(line);
        new_content.push('\n');

        if !replaced && line.contains("# CoolerBoost") {
            while let Some(next) = lines.peek() {
                let trimmed = next.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    new_content.push_str(next);
                    new_content.push('\n');
                    lines.next();
                    continue;
                }
                if next.starts_with("bindd = ") {
                    lines.next(); // consume old bind line
                    new_content.push_str(&new_line);
                    new_content.push('\n');
                    replaced = true;
                }
                break;
            }
        }
    }

    if !replaced {
        return Err("CoolerBoost binding block not found".into());
    }

    fs::write(&path, new_content)?;

    // Reload hyprland so the new shortcut takes effect immediately.
    let _ = Command::new("hyprctl").arg("reload").output();

    Ok(())
}

/// Toggles the `CoolerBoost` state.
///
/// Executes `isw -b on` or `isw -b off` via `sudo`, updates [`STATE_FILE`]
/// accordingly and shows a desktop notification. Errors from the underlying
/// commands are intentionally ignored because this tool is best-effort.
///
/// # Returns
/// The new state: `true` for ON, `false` for OFF.
#[must_use]
pub fn toggle() -> bool {
    let (was_enabled, command) = if check_status() {
        (true, ["isw", "-b", "off"])
    } else {
        (false, ["isw", "-b", "on"])
    };

    let _ = Command::new("sudo").args(command).output();

    if was_enabled {
        let _ = fs::remove_file(STATE_FILE);
        show_notification("CoolerBoost OFF", "Fan boost disabled");
        false
    } else {
        let _ = fs::File::create(STATE_FILE);
        show_notification("CoolerBoost ON", "Fan boost enabled");
        true
    }
}

/// Shows a desktop notification with the given title and body.
///
/// The notification is displayed on a background thread and automatically
/// dismissed after two seconds.
///
/// # Arguments
/// * `title` - Notification summary.
/// * `body` - Notification body text.
pub fn show_notification(title: &str, body: &str) {
    let title = title.to_string();
    let body = body.to_string();
    std::thread::spawn(move || {
        let _ = Notification::new()
            .summary(&title)
            .body(&body)
            .timeout(std::time::Duration::from_secs(2))
            .show();
    });
}

/// Generates a square RGBA icon representing the current `CoolerBoost` state.
///
/// Produces a smooth circle. When `enabled` is `true` the circle is green
/// (`#4CAF50`); otherwise it is gray (`#757575`). Pixels outside the circle
/// are transparent.
///
/// # Arguments
/// * `enabled` - Whether `CoolerBoost` is currently enabled.
/// * `size` - Width and height of the generated icon in pixels.
///
/// # Returns
/// A flat `Vec<u8>` containing the RGBA pixel data, row by row.
#[must_use]
pub fn create_icon_rgba(enabled: bool, size: u32) -> Vec<u8> {
    let (r, g, b) = if enabled {
        (76, 175, 80) // Green
    } else {
        (117, 117, 117) // Gray
    };

    let mut pixels = vec![0u8; (size * size * 4) as usize];
    let half = f64::from(size) / 2.0;
    let inner = f64::from(size) * 0.44;
    let outer = f64::from(size) * 0.47;

    for y in 0..size {
        for x in 0..size {
            let dx = f64::from(x) - half;
            let dy = f64::from(y) - half;
            let dist = (dx * dx + dy * dy).sqrt();

            let alpha = if dist < inner {
                255.0
            } else if dist < outer {
                (outer - dist) * 255.0
            } else {
                0.0
            };

            // The antialiasing factor is geometrically bounded to [0, 255],
            // so the truncation and sign-loss casts below are safe.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let a = alpha as u8;

            let idx = ((y * size + x) * 4) as usize;
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = a;
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn temp_config(content: &str) -> (PathBuf, PathBuf) {
        let dir = std::env::temp_dir().join(format!(
            "msi-coolerboost-tests-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = dir.join("hypr/bindings.conf");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
        (dir, path)
    }

    #[test]
    fn get_current_shortcut_parses_binding() {
        let _guard = ENV_LOCK.lock().unwrap();
        let (dir, _) = temp_config("# CoolerBoost Fan Toggle\nbindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost toggle\n");
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        assert_eq!(get_current_shortcut(), "SUPER CTRL + F");
    }

    #[test]
    fn get_current_shortcut_skips_blank_and_comment_lines() {
        let _guard = ENV_LOCK.lock().unwrap();
        let (dir, _) = temp_config(
            "# CoolerBoost Fan Toggle\n\n# another comment\nbindd = SUPER SHIFT, F10, Toggle CoolerBoost, exec, msi-coolerboost toggle\n",
        );
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        assert_eq!(get_current_shortcut(), "SUPER SHIFT + F10");
    }

    #[test]
    fn get_current_shortcut_returns_unknown_when_missing() {
        let _guard = ENV_LOCK.lock().unwrap();
        let (dir, _) =
            temp_config("# Some other binding\nbindd = SUPER, A, Something, exec, foo\n");
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        assert_eq!(get_current_shortcut(), "Unknown");
    }

    #[test]
    fn set_shortcut_updates_binding() {
        let _guard = ENV_LOCK.lock().unwrap();
        let (dir, path) = temp_config("# CoolerBoost Fan Toggle\nbindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost toggle\n");
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        set_shortcut("SUPER SHIFT", "F10").unwrap();
        let content = std::fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "bindd = SUPER SHIFT, F10, Toggle CoolerBoost, exec, msi-coolerboost toggle",
        ));
    }

    #[test]
    fn set_shortcut_skips_blank_and_comment_lines() {
        let _guard = ENV_LOCK.lock().unwrap();
        let (dir, path) = temp_config(
            "# CoolerBoost Fan Toggle\n\n# comment\nbindd = SUPER CTRL, F, Toggle CoolerBoost, exec, msi-coolerboost toggle\n",
        );
        std::env::set_var("XDG_CONFIG_HOME", &dir);
        set_shortcut("ALT", "X").unwrap();
        let content = std::fs::read_to_string(path).unwrap();
        assert!(
            content.contains("bindd = ALT, X, Toggle CoolerBoost, exec, msi-coolerboost toggle")
        );
    }

    #[test]
    fn create_icon_rgba_produces_expected_size() {
        let data = create_icon_rgba(true, 64);
        assert_eq!(data.len(), 64 * 64 * 4);
    }
}
