use std::{
    hash::Hash,
    ops::AddAssign,
    sync::{Arc, Mutex},
};

use egui::{Context, Pos2, WidgetWithState};

use crate::{AppState, Command, app_state};

#[derive(Debug, Clone)]
pub struct MenuState {
    current_selection: usize,
    text_input: String,
}

impl Default for MenuState {
    fn default() -> Self {
        Self {
            current_selection: 0,
            text_input: String::new(),
        }
    }
}

impl MenuState {
    pub fn load(ctx: &Context, id: egui::Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(id))
    }

    pub fn reset(ctx: &Context, id: egui::Id) {
        ctx.data_mut(|d| d.remove::<MenuState>(id))
    }

    pub fn store(self, ctx: &Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_persisted(id, self))
    }
}

#[derive(Debug)]
pub enum MenuAnchor {
    Top,
    TopLeftAligned(Pos2),
    TopRightAligned(Pos2),
}

#[derive(Debug)]
pub struct MenuEntry {
    text: String,
}

impl From<String> for MenuEntry {
    fn from(value: String) -> Self {
        MenuEntry { text: value }
    }
}

#[derive(Debug)]
pub struct Menu {
    entries: Vec<MenuEntry>,
    // anchor: MenuAnchor,
    // id: Option<egui::Id>,
}

impl WidgetWithState for Menu {
    type State = MenuState;
}

impl Menu {
    const ID: &str = "Menu";

    pub fn new(entries: Vec<MenuEntry>) -> Self {
        Self {
            entries,
            // anchor: MenuAnchor::Top,
        }
    }
}

impl egui::Widget for Menu {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = egui::Id::new(Self::ID);

        egui::Popup::new(
            id,
            ui.ctx().to_owned(),
            egui::PopupAnchor::Position(Pos2 { x: 50.0, y: 50.0 }),
            egui::LayerId::new(egui::Order::Foreground, id),
        )
        .show(|ui| {
            let mut state = MenuState::load(ui.ctx(), id).unwrap_or_default();
            ui.text_edit_singleline(&mut state.text_input);

            for (i, entry) in self.entries.iter().enumerate() {
                if i == state.current_selection {
                    ui.label(entry.text.clone()).highlight();
                } else {
                    ui.label(entry.text.clone());
                }
            }

            state.store(ui.ctx(), id);
        });

        ui.response()
    }
}
