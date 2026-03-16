use crate::CommandPalette;

#[derive(Debug)]
pub struct AppState {
    pub inspect: bool,
    pub command_palette: CommandPalette,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inspect: true,
            command_palette: CommandPalette::new(),
        }
    }
}
