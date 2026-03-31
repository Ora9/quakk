use gpui::prelude::*;
use gpui::{Window, div};

pub struct PaletteMenu {
    selected: usize,
}

impl Render for PaletteMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
