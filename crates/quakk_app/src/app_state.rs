use crate::Menu;

#[derive(Default, Debug)]
pub struct AppState {
    pub inspect: bool,
    pub command_panel_opened: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inspect: true,
            command_panel_opened: true,
        }
    }
}
