use std::{hash::Hash, ops::AddAssign};

use egui::{Context, Id, Pos2, WidgetWithState};

#[derive(Debug, Clone)]
pub struct CommandPaletteState {
    current_selection: usize,
    text_input: String,
}

impl Default for CommandPaletteState {
    fn default() -> Self {
        Self {
            current_selection: 0,
            text_input: String::new(),
        }
    }
}

impl CommandPaletteState {
    pub fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(id))
    }

    pub fn reset(ctx: &Context, id: Id) {
        ctx.data_mut(|d| d.remove::<CommandPaletteState>(id))
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|d| d.insert_persisted(id, self))
    }
}

#[derive(Debug)]
pub struct CommandPalette;

impl WidgetWithState for CommandPalette {
    type State = CommandPaletteState;
}

impl CommandPalette {
    const ID: &str = "CommandPalette";

    pub fn new() -> Self {
        Self
    }
}

impl egui::Widget for CommandPalette {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let id = egui::Id::new(Self::ID);

        egui::Popup::new(
            id,
            ui.ctx().to_owned(),
            egui::PopupAnchor::Position(Pos2 { x: 50.0, y: 50.0 }),
            egui::LayerId::new(egui::Order::Foreground, id),
        )
        .show(|ui| {
            let mut state = CommandPaletteState::load(ui.ctx(), id).unwrap_or_default();
            ui.text_edit_singleline(&mut state.text_input);

            state.store(ui.ctx(), id);
        });

        ui.response()
    }
}
