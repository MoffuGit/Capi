use leptos::prelude::*;
use leptos_meta::{Body, Html};

use crate::api::ToggleTheme;

#[derive(Clone)]
pub struct ThemeContext {
    pub toggle_theme: ServerAction<ToggleTheme>,
    pub prefers_theme: Signal<bool>,
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
    provide_context(ThemeContext {
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
        <Body {..} class=move || format!("w-full h-screen font-geist {}", theme()) />
        {children()}
    }
}

// #[component]
// pub fn ThemeToggle(
//     class: &'static str,
//     // #[prop(optional)] icons: Option<ThemeIcons>,
// ) -> impl IntoView {
//     let theme_context = use_theme();
//     let toggle_theme = theme_context.toggle_theme;
//     let prefers_theme = theme_context.prefers_theme;
//
//     view! {
//         <ActionForm action=toggle_theme>
//             <input type="hidden" name="theme" value=move || (!prefers_theme.get()).to_string() />
//             <button type="submit" class="w-10 h-10 flex items-center justify-center bg-primary">
//                 // {icons
//                 //     .clone()
//                 //     .map(|icons| move || match prefers_theme.get() {
//                 //         true => view! { <Icon icon=icons.dark /> },
//                 //         false => view! { <Icon icon=icons.light /> },
//                 //     })}
//             </button>
//         </ActionForm>
//     }
// }
