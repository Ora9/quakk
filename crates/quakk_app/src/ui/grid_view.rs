use gpui::prelude::*;
use gpui::{SharedString, Window, div};

pub struct GraphView {
    pub text: SharedString,
}

impl Render for GraphView {
    fn render(&mut self, _window: &mut Window, _cxx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .child(self.text.clone())
    }
}
