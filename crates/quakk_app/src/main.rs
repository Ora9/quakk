use gpui::{App, Context, SharedString, TitlebarOptions, Window, WindowOptions, div, prelude::*};
use gpui_component::menu::{ContextMenuExt, PopupMenuItem};
use gpui_component::{IconName, TitleBar};
use gpui_component::{Root, button::Button};
use gpui_component_assets::Assets;


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

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let quakk_app = cx.new(|_| QuakkApp::new());
                cx.new(|cx| Root::new(quakk_app, window, cx))
            })
            .expect("Failed to open window")
        })
        .detach();
    });
}
