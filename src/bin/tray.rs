//! System tray application for MSI CoolerBoost.
//!
//! Registers a tray icon that reflects the current CoolerBoost state. Clicking
//! the icon toggles the state, and the right-click menu displays the current
//! Hyprland shortcut plus a quit option.

use ksni::{self, blocking::TrayMethods, menu::StandardItem, MenuItem, ToolTip};

/// Represents the current tray state for MSI CoolerBoost.
#[derive(Debug)]
struct TrayState {
    /// Whether CoolerBoost is currently enabled.
    enabled: bool,
}

impl ksni::Tray for TrayState {
    fn id(&self) -> String {
        "msi-coolerboost".into()
    }

    fn icon_name(&self) -> String {
        "msi-coolerboost".into()
    }

    fn title(&self) -> String {
        "MSI CoolerBoost".into()
    }

    /// Called when the user clicks the tray icon. Toggles CoolerBoost and
    /// refreshes the cached state.
    fn activate(&mut self, _x: i32, _y: i32) {
        msi_coolerboost::toggle();
        self.enabled = msi_coolerboost::check_status();
    }

    /// Returns the icon shown in the system tray.
    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        vec![self.create_icon()]
    }

    /// Returns the tooltip shown when hovering over the tray icon.
    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "MSI CoolerBoost".into(),
            description: format!("CoolerBoost: {}", if self.enabled { "ON" } else { "OFF" }),
            icon_name: "msi-coolerboost".into(),
            icon_pixmap: vec![self.create_icon()],
        }
    }

    /// Builds the tray context menu.
    fn menu(&self) -> Vec<MenuItem<Self>> {
        let shortcut = msi_coolerboost::get_current_shortcut();

        vec![
            MenuItem::Standard(StandardItem {
                label: format!("Shortcut: {}", shortcut),
                enabled: true,
                activate: Box::new(|_: &mut Self| {
                    msi_coolerboost::show_notification(
                        "Shortcut Change",
                        "Edit ~/.config/hypr/bindings.conf",
                    );
                }),
                ..Default::default()
            }),
            MenuItem::Separator,
            MenuItem::Standard(StandardItem {
                label: "Quit".into(),
                enabled: true,
                activate: Box::new(|_: &mut Self| {
                    std::process::exit(0);
                }),
                ..Default::default()
            }),
        ]
    }
}

impl TrayState {
    /// Creates a system tray icon from the current state.
    ///
    /// Converts RGBA pixel data from [`create_icon_rgba`](msi_coolerboost::create_icon_rgba)
    /// into ARGB format required by the system tray, with the alpha channel in
    /// the first byte.
    fn create_icon(&self) -> ksni::Icon {
        let rgba = msi_coolerboost::create_icon_rgba(self.enabled, 64);
        let data: Vec<u8> = rgba
            .chunks(4)
            .flat_map(|c| [c[3], c[0], c[1], c[2]])
            .collect();
        ksni::Icon {
            width: 64,
            height: 64,
            data,
        }
    }
}

/// Entry point for the MSI CoolerBoost system tray application.
///
/// Initializes the tray state from the current CoolerBoost status and starts
/// the tray service event loop. The main thread parks indefinitely so the tray
/// icon remains active until the user quits.
fn main() {
    let enabled = msi_coolerboost::check_status();
    TrayState { enabled }.spawn().expect("failed to spawn tray");
    std::thread::park();
}
