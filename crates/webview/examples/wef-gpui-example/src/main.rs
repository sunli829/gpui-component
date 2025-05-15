use std::time::Duration;

use gpui::{
    App, AppContext, Application, Bounds, Context, Entity, IntoElement, ParentElement, Render,
    Styled, Timer, Window, WindowBounds, WindowOptions, div, px, size,
};
use gpui_component::Root;
use gpui_webview::{
    WebView,
    event::TitleChangedEvent,
    wef::{self, FuncRegistry, Settings},
};

struct Main {
    webview1: Entity<WebView>,
}

impl Main {
    fn new(window: &mut Window, cx: &mut App) -> Entity<Self> {
        let index_path = std::env::current_dir()
            .unwrap()
            .join("crates/webview/tests/index.html")
            .display()
            .to_string();

        let background_executor = cx.background_executor().clone();

        let func_registry = FuncRegistry::build()
            .with_spawner(move |fut| {
                background_executor.spawn(fut).detach();
            })
            .register("toUppercase", |value: String| value.to_uppercase())
            .register("addInt", |a: i32, b: i32| a + b)
            .register("parseInt", |value: String| value.parse::<i32>())
            .register_async("sleep", |millis: u64| async move {
                Timer::after(Duration::from_millis(millis)).await;
                "ok"
            })
            .build();

        let webview1 = WebView::with_func_registry(&index_path, func_registry.clone(), window, cx);

        window
            .subscribe(&webview1, cx, |_, event: &TitleChangedEvent, window, _| {
                window.set_window_title(&event.title);
            })
            .detach();

        cx.new(|_| Self { webview1 })
    }
}

impl Render for Main {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.webview1.clone())
            .children(Root::render_modal_layer(window, cx))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if wef::exec_process()? {
        return Ok(());
    }

    let settings = Settings::new();
    wef::init(settings)?;

    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let main = Main::new(window, cx);
                cx.new(|cx| Root::new(main.into(), window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });

    wef::shutdown();
    Ok(())
}
