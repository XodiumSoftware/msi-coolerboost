use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
    TrayIconBuilder,
};
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};

struct AppState {
    #[allow(dead_code)]
    tray: tray_icon::TrayIcon,
    menu_items: MenuItems,
}

#[derive(Clone)]
struct MenuItems {
    toggle_id: MenuId,
    change_id: MenuId,
    quit_id: MenuId,
}

struct App {
    state: Option<AppState>,
}

impl App {
    fn new() -> Self {
        Self { state: None }
    }

    fn create_tray(is_enabled: bool) -> (tray_icon::TrayIcon, MenuItems) {
        let rgba = msi_coolerboost::create_icon_rgba(is_enabled, 64);
        let icon = tray_icon::Icon::from_rgba(rgba, 64, 64).expect("Failed to create icon");

        let menu = Menu::new();

        let toggle_item = MenuItem::new("Toggle CoolerBoost", true, None);
        let sep1 = PredefinedMenuItem::separator();
        let status = if is_enabled { "ON" } else { "OFF" };
        let status_item = MenuItem::new(format!("Status: {}", status), false, None);
        let shortcut = msi_coolerboost::get_current_shortcut();
        let shortcut_item = MenuItem::new(format!("Shortcut: {}", shortcut), false, None);
        let sep2 = PredefinedMenuItem::separator();
        let change_item = MenuItem::new("Change Shortcut...", true, None);
        let sep3 = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("Quit", true, None);

        menu.append(&toggle_item).unwrap();
        menu.append(&sep1).unwrap();
        menu.append(&status_item).unwrap();
        menu.append(&shortcut_item).unwrap();
        menu.append(&sep2).unwrap();
        menu.append(&change_item).unwrap();
        menu.append(&sep3).unwrap();
        menu.append(&quit_item).unwrap();

        let toggle_id = toggle_item.id().clone();
        let change_id = change_item.id().clone();
        let quit_id = quit_item.id().clone();

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_icon(icon)
            .with_tooltip(format!("CoolerBoost: {}", status))
            .build()
            .unwrap();

        let menu_items = MenuItems {
            toggle_id,
            change_id,
            quit_id,
        };

        (tray, menu_items)
    }
}

impl ApplicationHandler<MenuEvent> for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.state.is_none() {
            let is_enabled = msi_coolerboost::check_status();
            let (tray, menu_items) = Self::create_tray(is_enabled);

            self.state = Some(AppState { tray, menu_items });
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: MenuEvent) {
        if let Some(state) = &mut self.state {
            if event.id == state.menu_items.toggle_id {
                msi_coolerboost::toggle();
                // Recreate tray with new icon
                let is_enabled = msi_coolerboost::check_status();
                let (tray, menu_items) = Self::create_tray(is_enabled);
                *state = AppState { tray, menu_items };
            } else if event.id == state.menu_items.change_id {
                msi_coolerboost::show_notification(
                    "Shortcut Change",
                    "Edit ~/.config/hypr/bindings.conf",
                );
            } else if event.id == state.menu_items.quit_id {
                event_loop.exit();
            }
        }
    }
}

fn main() {
    // Initialize GTK for Linux system tray support
    #[cfg(target_os = "linux")]
    {
        gtk::init().expect("Failed to initialize GTK");
    }

    let event_loop = EventLoop::<MenuEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    MenuEvent::set_event_handler(Some(|_event: MenuEvent| {
        // Event is sent to user_event
    }));

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
