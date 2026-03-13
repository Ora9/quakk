#[derive(Debug)]
pub struct App {
    pub egui_ctx: egui::Context,
    pub start_time: std::time::Instant,
}

impl App {
    pub const APP_NAME: &str = "Quakk";

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            egui_ctx: cc.egui_ctx.clone(),
        }
    }

    pub fn ui(&mut self, egui_ctx: &egui::Context) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            ui.heading("Quakkk!");
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
