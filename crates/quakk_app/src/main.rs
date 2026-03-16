use eframe::egui;

mod app;
pub use app::App;

mod app_state;
pub use app_state::AppState;

mod command;
pub use command::Command;

mod components;
pub use components::menu::{Menu, MenuAnchor, MenuEntry};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        App::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
