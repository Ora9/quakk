#[derive(Debug, Default)]
pub struct AppState {
    pub inspect: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self { inspect: true }
    }
}
