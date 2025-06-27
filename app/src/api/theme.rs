use leptos::prelude::*;

#[server(ToggleTheme)]
pub async fn toggle_theme(theme: bool) -> Result<bool, ServerFnError> {
    use axum::http::{header::SET_COOKIE, HeaderMap, HeaderValue};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("theme={theme}; Path=/")).expect("to create header value"),
    );
    response_parts.headers = headers;

    response.overwrite(response_parts);
    Ok(theme)
}
