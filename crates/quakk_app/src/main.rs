use gpui::{
    App, AppContext, Entity, KeyBinding, TitlebarOptions, Window, WindowOptions, actions, div,
};
use gpui::{FocusHandle, prelude::*};
use gpui_component::menu::DropdownMenu;
use gpui_component::{
    Root,
    button::{Button, ButtonVariants},
    h_flex,
    menu::PopupMenuItem,
};
use gpui_component_assets::Assets;
use quakk_app::{GraphView, MenuBar, Picker, PickerItem};

actions!(quakk, [Quit, ToggleAbout, ToggleCommandPalette, Debug]);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-p", ToggleCommandPalette, None),
        KeyBinding::new("alt-a", ToggleAbout, None),
    ]);
}

// #[derive(Debug)]
pub struct QuakkApp {
    pub(crate) focus_handle: FocusHandle,

    command_palette: Option<Entity<Picker>>,
}

impl QuakkApp {
    pub const APP_TITLE: &'static str = "Quakk";
    pub const APP_ID: &'static str = "quakk";

    fn toggle_about(&mut self, _: &ToggleAbout, _window: &mut Window, _cx: &mut Context<Self>) {
        dbg!("toggle about");
    }

    fn toggle_command_palette(
        &mut self,
        _: &ToggleCommandPalette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        dbg!("toggle command_palette");

        self.command_palette = Some(cx.new(|cx| {
            Picker::new(
                vec![
                    PickerItem::new("Patate"),
                    PickerItem::new("Tomates"),
                    PickerItem::new("Oranges"),
                    PickerItem::new("Bananes"),
                ],
                window,
                cx,
            )
        }));

        // window.open_dialog(cx, move |dialog, _, _| {
        //     dialog.child(self.command_palette.clone().unwrap())
        //     // dialog.title("Test dialog").child("Hello from dialog!")
        // });
    }
}

impl Render for QuakkApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();

        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle_command_palette))
            .on_action(cx.listener(Self::toggle_about))
            .size_full()
            .child(
                MenuBar::new().child(
                    h_flex().child(
                        Button::new("menu_bar_quakk")
                            .label(Self::APP_TITLE)
                            .ghost()
                            .dropdown_menu(move |menu, window, _cx| {
                                menu.link("Github", "https://github.com/Ora9/quakk").item(
                                    PopupMenuItem::new("About").on_click(window.listener_for(
                                        &view,
                                        |_this, _, _window: &mut Window, _cx| {
                                            dbg!("about!");
                                        },
                                    )),
                                )
                            }),
                    ),
                ),
            )
            .child(cx.new(|_ctx| GraphView { text: "AHH".into() }))
            .children(Root::render_dialog_layer(window, cx))
            .children(Root::render_sheet_layer(window, cx))
    }
}
fn main() {
    let application = gpui_platform::application().with_assets(Assets);

    let window_options = WindowOptions {
        app_id: Some(QuakkApp::APP_ID.to_string()),
        titlebar: Some(TitlebarOptions {
            appears_transparent: true,
            title: Some(QuakkApp::APP_TITLE.to_string().into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    application.run(|cx: &mut App| {
        gpui_component::init(cx);
        init(cx);

        let focus_handle = cx.focus_handle();
        let quakk_app = cx.new(|_| QuakkApp {
            focus_handle: focus_handle,

            command_palette: None,
        });

        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("ctrl-q", Quit, None)]);

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                cx.new(|cx| Root::new(quakk_app, window, cx))
            })
            .expect("Failed to open window")
        })
        .detach();
    });
}
