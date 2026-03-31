use gpui::{App, AppContext, TitlebarOptions, WindowOptions};
use gpui_component::Root;
use gpui_component_assets::Assets;
use quakk_app::QuakkApp;

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
