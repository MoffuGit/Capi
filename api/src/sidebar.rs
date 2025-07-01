use leptos::prelude::*;
use serde::{Deserialize, Serialize};

pub const SIDEBAR_COOKIE_NAME: &str = "sidebar_state";
pub const SIDEBAR_COOKIE_MAX_AGE: usize = 60 * 60 * 24 * 7;

#[derive(Debug, PartialEq, Clone, strum_macros::Display, Serialize, Deserialize)]
pub enum SideBarState {
    #[strum(to_string = "expanded")]
    Expanded,
    #[strum(to_string = "collapsed")]
    Collapsed,
}

#[server(ToggleSideBar)]
pub async fn toggle_sidebar(state: SideBarState) -> Result<SideBarState, ServerFnError> {
    use axum::http::{HeaderMap, HeaderValue, header::SET_COOKIE};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>().expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!(
            "{SIDEBAR_COOKIE_NAME}={state}; max-age=${SIDEBAR_COOKIE_MAX_AGE}; Path=/;"
        ))
        .expect("to create header value"),
    );
    response_parts.headers = headers;

    response.overwrite(response_parts);
    Ok(state)
}
