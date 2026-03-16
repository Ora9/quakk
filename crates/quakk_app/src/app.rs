use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use egui::{Align2, Context, Widget};

use crate::{AppState, CommandPalette};

#[derive(Debug)]
pub struct App {
    pub start_time: std::time::Instant,
    pub egui_ctx: egui::Context,

    pub app_state: Arc<Mutex<AppState>>,

    pub tiling_behavior: TilingBehavior,
    pub tiling_tree: egui_tiles::Tree<View>,
}

impl eframe::App for App {
    fn update(&mut self, egui_ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.ui(egui_ctx);
    }
}

impl App {
    pub const APP_NAME: &str = "Quakk";

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app_state = Arc::new(Mutex::new(AppState::new()));

        Self {
            egui_ctx: cc.egui_ctx.clone(),

            tiling_behavior: TilingBehavior::new(app_state.clone()),
            tiling_tree: Self::initialize_tiling_tree(),

            app_state: app_state,
            start_time: std::time::Instant::now(),
        }
    }

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

    fn handle_input(&mut self, egui_ctx: &Context) {
        // make a per view setting to catch or not nav-keys
        // let event_filter = egui::EventFilter {
        //     horizontal_arrows: true,
        //     vertical_arrows: true,
        //     escape: true,
        //     tab: true,
        // };
        // ctx.memory_mut(|mem| mem.set_focus_lock_filter(grid_id, event_filter));
        // ctx.input(|i| i.filtered_events(&event_filter))

        let events = egui_ctx.input(|i| i.events.to_owned());

        for event in events {}
    }

    pub fn ui(&mut self, egui_ctx: &egui::Context) {
        egui::CentralPanel::default().show(egui_ctx, |ui| {
            self.tiling_tree.ui(&mut self.tiling_behavior, ui);

            if let Ok(app_state) = self.app_state.lock() {
                // ui.add(app_state.command_palette);
            } else {
                panic!("aah can't lock the state")
            }
        });
    }
}

#[derive(Debug)]
pub struct View {
    title: String,
}

#[derive(Debug)]
pub struct TilingBehavior {
    app_state: Arc<Mutex<AppState>>,
}

impl TilingBehavior {
    fn new(app_state: Arc<Mutex<AppState>>) -> Self {
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

        if let Ok(mut app_state) = self.app_state.lock() {
            ui.checkbox(&mut app_state.inspect, "ouais");

            if app_state.inspect {
                CommandPalette::new().ui(ui);
            }
        } else {
            ui.label("can't .. sry");
        }

        Default::default()
    }
}
