//! Core library for MSI CoolerBoost.
//!
//! Provides shared functionality for checking, toggling and displaying the
//! CoolerBoost fan-boost state on MSI laptops through the `isw` tool. Also
//! includes helpers for parsing and updating the Hyprland keybinding as well
//! as generating a status icon and desktop notifications.

use notify_rust::Notification;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Path to the state file used to mirror the current CoolerBoost state.
///
/// The file is created when CoolerBoost is ON and removed when OFF. It lives
/// on tmpfs so it is automatically cleared on reboot.
pub const STATE_FILE: &str = "/tmp/isw_coolerboost";

/// Path to the Hyprland bindings file, relative to the user's home directory.
pub const BINDINGS_FILE: &str = ".config/hypr/bindings.conf";

/// Checks whether CoolerBoost is currently enabled.
///
/// This is determined by the presence of [`STATE_FILE`].
///
/// # Returns
/// `true` if the state file exists, otherwise `false`.
pub fn check_status() -> bool {
    PathBuf::from(STATE_FILE).exists()
}

/// Retrieves the currently configured CoolerBoost shortcut from Hyprland bindings.
///
/// Parses `~/.config/hypr/bindings.conf` looking for a comment line containing
/// `CoolerBoost` followed by a `bindd = ...` entry and returns a formatted
/// string such as `"SUPER + F10"`.
///
/// # Returns
/// The parsed shortcut, or `"Unknown"` if parsing fails.
pub fn get_current_shortcut() -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let bindings_path = PathBuf::from(home).join(BINDINGS_FILE);

    if let Ok(content) = fs::read_to_string(&bindings_path) {
        let re = Regex::new(r"# CoolerBoost.*\nbindd\s*=\s*(.+?),\s*(\w+),").unwrap();
        if let Some(caps) = re.captures(&content) {
            return format!("{} + {}", &caps[1], &caps[2]);
        }
    }
    "Unknown".to_string()
}

/// Updates the CoolerBoost keybinding in the Hyprland config.
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
/// Returns an error if the `HOME` environment variable is missing, the
/// bindings file cannot be read, or the updated file cannot be written.
pub fn set_shortcut(modifiers: &str, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let bindings_path = PathBuf::from(home).join(BINDINGS_FILE);
    let content = fs::read_to_string(&bindings_path)?;

    let new_line = format!(
        "bindd = {}, {}, Toggle CoolerBoost, exec, msi-coolerboost toggle",
        modifiers,
        key.to_uppercase()
    );

    let re = Regex::new(r"(# CoolerBoost Fan Toggle\n)bindd\s*=\s*.+?\n").unwrap();
    let new_content = re.replace(&content, |caps: &regex::Captures| {
        format!("{} {}\n", &caps[1], new_line)
    });

    fs::write(&bindings_path, new_content.as_ref())?;

    // Reload hyprland so the new shortcut takes effect immediately.
    let _ = Command::new("hyprctl").arg("reload").output();

    Ok(())
}

/// Toggles the CoolerBoost state.
///
/// Executes `isw -b on` or `isw -b off` via `sudo`, updates [`STATE_FILE`]
/// accordingly and shows a desktop notification. Errors from the underlying
/// commands are intentionally ignored because this tool is best-effort.
///
/// # Returns
/// The new state: `true` for ON, `false` for OFF.
pub fn toggle() -> bool {
    if check_status() {
        let _ = Command::new("sudo").args(["isw", "-b", "off"]).output();
        let _ = fs::remove_file(STATE_FILE);
        show_notification("CoolerBoost OFF", "Fan boost disabled");
        false
    } else {
        let _ = Command::new("sudo").args(["isw", "-b", "on"]).output();
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

/// Generates a square RGBA icon representing the current CoolerBoost state.
///
/// Produces a smooth circle. When `enabled` is `true` the circle is green
/// (`#4CAF50`); otherwise it is gray (`#757575`). Pixels outside the circle
/// are transparent.
///
/// # Arguments
/// * `enabled` - Whether CoolerBoost is currently enabled.
/// * `size` - Width and height of the generated icon in pixels.
///
/// # Returns
/// A flat `Vec<u8>` containing the RGBA pixel data, row by row.
pub fn create_icon_rgba(enabled: bool, size: u32) -> Vec<u8> {
    use image::{ImageBuffer, Rgba};

    let (r, g, b) = if enabled {
        (76, 175, 80) // Green
    } else {
        (117, 117, 117) // Gray
    };

    let mut img = ImageBuffer::new(size, size);

    for y in 0..size {
        for x in 0..size {
            let dx = x as i32 - (size as i32 / 2);
            let dy = y as i32 - (size as i32 / 2);
            let dist = ((dx * dx + dy * dy) as f64).sqrt();

            if dist < (size as f64 * 0.44) {
                img.put_pixel(x, y, Rgba([r, g, b, 255]));
            } else if dist < (size as f64 * 0.47) {
                let alpha = ((size as f64 * 0.47 - dist) * 255.0) as u8;
                img.put_pixel(x, y, Rgba([r, g, b, alpha]));
            } else {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    img.into_raw()
}
