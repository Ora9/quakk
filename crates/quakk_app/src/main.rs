use gpui::{
    App, AppContext, Entity, KeyBinding, TitlebarOptions, Window, WindowOptions, actions, div,
};
use gpui::{FocusHandle, prelude::*};
use gpui_component::menu::DropdownMenu;
use gpui_component::{
    Root, WindowExt,
    button::{Button, ButtonVariants},
    h_flex,
    menu::PopupMenuItem,
};
use gpui_component_assets::Assets;
use quakk_app::{GraphView, MenuBar, Picker, PickerItem};

actions!(quakk, [Quit, ShowAbout, ShowCommandPalette, Debug]);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("ctrl-p", ShowCommandPalette, None),
        KeyBinding::new("alt-a", ShowAbout, None),
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

    fn show_about(&mut self, _: &ShowAbout, window: &mut Window, cx: &mut Context<Self>) {
        dbg!("about");
    }

    fn show_command_palette(
        &mut self,
        _: &ShowCommandPalette,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        dbg!("command_palette");

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
            .on_action(cx.listener(Self::show_command_palette))
            .on_action(cx.listener(Self::show_about))
            .size_full()
            .child(
                MenuBar::new().child(
                    h_flex().child(
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
            .child(cx.new(|ctx| GraphView { text: "AHH".into() }))
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
