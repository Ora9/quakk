use gpui::{Window, div, prelude::*};
use gpui_component::{
    IconName, Root, TitleBar,
    button::{Button, ButtonVariants},
    menu::{DropdownMenu, PopupMenuItem},
};

use crate::{GraphView, MenuBar};

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

        // use gpui_component::popover::{PopupMenuExt as _, PopupMenuItem};

        let view = cx.entity();

        div()
            .size_full()
            // .child(sidebar)
            .child(
                MenuBar::new().child(
                    div().child(
                        Button::new("menu_bar_quakk")
                            .label(Self::APP_TITLE)
                            .ghost()
                            .dropdown_menu(move |menu, window, cx| {
                                menu.link("Github", "https://github.com/Ora9/quakk").item(
                                    PopupMenuItem::new("About").on_click(window.listener_for(
                                        &view,
                                        |this, _, window: &mut Window, cx| {
                                            dbg!("about!");
                                        },
                                    )),
                                )
                            }),
                    ),
                ),
            )
            .child(graph_view)
            // Render the dialog layer on top of the app content
            .children(sheet_layer)
            .children(dialog_layer)
    }
}
