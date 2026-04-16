use gpui::{AnyElement, div, prelude::*, px};
use gpui_component::ActiveTheme;

#[derive(IntoElement)]
pub struct MenuBar {
    childrens: Vec<AnyElement>,
}

impl ParentElement for MenuBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.childrens.extend(elements);
    }
}

impl MenuBar {
    pub fn new() -> Self {
        MenuBar {
            childrens: Vec::new(),
        }
    }
}

impl RenderOnce for MenuBar {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        div().flex_shrink_0().child(
            div()
                .id("title_bar")
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(34.0))
                .p(px(12.0))
                .border_b_1()
                .border_color(cx.theme().title_bar_border)
                .bg(cx.theme().title_bar)
                .children(self.childrens),
        )
    }
}
