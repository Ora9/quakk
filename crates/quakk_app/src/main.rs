use eframe::egui;

mod app;
pub use app::App;

mod app_state;
pub use app_state::AppState;

mod viewport;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        App::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
