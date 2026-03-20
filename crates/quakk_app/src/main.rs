use eframe::egui;
use quakk_app::App;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        App::APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    );
}
