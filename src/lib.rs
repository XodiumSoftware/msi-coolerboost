use notify_rust::Notification;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const STATE_FILE: &str = "/tmp/isw_coolerboost";
pub const BINDINGS_FILE: &str = ".config/hypr/bindings.conf";

/// Checks whether CoolerBoost is currently enabled.
///
/// This is determined by the presence of a state file.
pub fn check_status() -> bool {
    PathBuf::from(STATE_FILE).exists()
}

/// Retrieves the currently configured CoolerBoost shortcut from Hyprland bindings.
///
/// Returns a formatted string like "SUPER + F10".
/// Falls back to "Unknown" if parsing fails.
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

/// Updates the CoolerBoost keybinding in Hyprland config.
///
/// # Arguments
/// * `modifiers` - Modifier keys (e.g. "SUPER")
/// * `key` - Key to bind (e.g. "F10")
///
/// # Errors
/// Returns an error if the config file cannot be read or written.
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

    // Reload hyprland
    let _ = Command::new("hyprctl").arg("reload").output();

    Ok(())
}

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

pub fn show_notification(title: &str, body: &str) {
    let _ = Notification::new()
        .summary(title)
        .body(body)
        .timeout(std::time::Duration::from_secs(2))
        .show();
}

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
