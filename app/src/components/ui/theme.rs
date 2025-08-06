use api::theme::ToggleTheme;
use leptos::prelude::*;
use leptos_meta::{Body, Html};

#[derive(Clone)]
pub struct ThemeContext {
    toggle_theme: ServerAction<ToggleTheme>,
    pub prefers_theme: Signal<bool>,
    pub select_theme: Callback<&'static bool>,
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("theme context")
}

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark() -> bool {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    cookie.contains("theme=true")
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> bool {
    use axum_extra::extract::cookie::CookieJar;
    use_context::<http::request::Parts>()
        .and_then(|req| {
            let cookies = CookieJar::from_headers(&req.headers);
            cookies.get("theme").and_then(|v| match v.value() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            })
        })
        .unwrap_or(false)
}
#[component]
pub fn ThemeProvider(children: ChildrenFn) -> impl IntoView {
    let initial = initial_prefers_dark();
    let toggle_theme = ServerAction::<ToggleTheme>::new();
    let input = toggle_theme.input();
    let value = toggle_theme.value();
    let prefers_theme = Signal::derive(move || match (input.get(), value.get()) {
        (Some(submission), _) => submission.theme,
        (_, Some(Ok(value))) => value,
        _ => initial,
    });
    let select_theme = Callback::new(move |theme: &bool| {
        toggle_theme.dispatch(ToggleTheme { theme: *theme });
    });
    provide_context(ThemeContext {
        select_theme,
        toggle_theme,
        prefers_theme,
    });
    let theme = move || {
        if prefers_theme.get() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };
    view! {
        <Html attr:data-theme=theme />
        <Body {..} class=move || format!("w-full h-screen font-inter overflow-hidden {}", theme()) />
        {children()}
    }
}
