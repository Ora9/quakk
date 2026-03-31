use gpui::{Window, div, prelude::*};
use gpui_component::{IconName, Root, TitleBar, button::Button};

use crate::GraphView;

pub struct QuakkApp {}

impl QuakkApp {
    pub const APP_TITLE: &'static str = "Quakk";
    pub const APP_ID: &'static str = "quakk";

    pub fn new() -> Self {
        Self {}
    }
}

impl Render for QuakkApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dialog_layer = Root::render_dialog_layer(window, cx);
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let graph_view = cx.new(|ctx| GraphView { text: "AHH".into() });

        div()
            .size_full()
            // .child(sidebar)
            .child(
                TitleBar::new().child(QuakkApp::APP_TITLE).child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(Button::new("settings").icon(IconName::Settings))
                        .child(Button::new("profile").icon(IconName::User)),
                ),
            )
            .child(graph_view)
            // Render the dialog layer on top of the app content
            .children(sheet_layer)
            .children(dialog_layer)
    }
}
