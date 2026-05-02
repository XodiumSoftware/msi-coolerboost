use ksni::{self, blocking::TrayMethods, menu::StandardItem, MenuItem, ToolTip};

/// Represents the current tray state for MSI CoolerBoost.
#[derive(Debug)]
struct TrayState {
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

    fn activate(&mut self, _x: i32, _y: i32) {
        msi_coolerboost::toggle();
        self.enabled = msi_coolerboost::check_status();
    }

    fn icon_pixmap(&self) -> Vec<ksni::Icon> {
        vec![self.create_icon()]
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "MSI CoolerBoost".into(),
            description: format!("CoolerBoost: {}", if self.enabled { "ON" } else { "OFF" }),
            icon_name: "msi-coolerboost".into(),
            icon_pixmap: vec![self.create_icon()],
        }
    }

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
    /// Converts RGBA pixel data from `create_icon_rgba` into ARGB format
    /// required by the system tray, with alpha channel in the first byte.
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
/// Initializes the tray state from the current CoolerBoost status
/// and starts the tray service event loop.
fn main() {
    let enabled = msi_coolerboost::check_status();
    TrayState { enabled }.spawn().expect("failed to spawn tray");
    std::thread::park();
}
