use std::rc::Rc;

use egui::Align2;

use crate::AppState;

#[derive(Debug)]
pub struct View {
    title: String,
}

#[derive(Debug)]
pub struct TilingBehavior {
    app_state: Rc<AppState>,
}

impl TilingBehavior {
    fn new(app_state: Rc<AppState>) -> Self {
        Self { app_state }
    }
}

impl egui_tiles::Behavior<View> for TilingBehavior {
    fn tab_title_for_pane(&mut self, pane: &View) -> egui::WidgetText {
        pane.title.to_owned().into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        pane: &mut View,
    ) -> egui_tiles::UiResponse {
        ui.label(pane.title.clone());

        Default::default()
    }
}

#[derive(Debug)]
pub struct App {
    pub egui_ctx: egui::Context,
    pub app_state: Rc<AppState>,
    pub tiling_behavior: TilingBehavior,
    pub tiling_tree: egui_tiles::Tree<View>,
    pub start_time: std::time::Instant,
}

impl App {
    pub const APP_NAME: &str = "Quakk";

    fn initialize_tiling_tree() -> egui_tiles::Tree<View> {
        let mut tiles = egui_tiles::Tiles::default();

        let graph = tiles.insert_pane(View {
            title: "Graph".to_string(),
        });
        let output = tiles.insert_pane(View {
            title: "Output".to_string(),
        });

        let horizontal = tiles.insert_container({
            let mut linear = egui_tiles::Linear {
                children: vec![graph, output],
                dir: egui_tiles::LinearDir::Horizontal,
                ..Default::default()
            };

            linear.shares.set_share(graph, 0.7);
            linear.shares.set_share(output, 0.3);

            linear
        });

        egui_tiles::Tree::new(
            format!("{}RootTilingTree", Self::APP_NAME),
            horizontal,
            tiles,
        )
    }

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app_state = Rc::new(AppState::default());

        Self {
            egui_ctx: cc.egui_ctx.clone(),

            tiling_behavior: TilingBehavior::new(app_state.clone()),
            tiling_tree: Self::initialize_tiling_tree(),

            app_state: app_state,
            start_time: std::time::Instant::now(),
        }
    }

    pub fn ui(&mut self, egui_ctx: &egui::Context) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            self.tiling_tree.ui(&mut self.tiling_behavior, ui);
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(ctx);
    }
}
