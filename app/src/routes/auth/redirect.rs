use leptos::prelude::*;

use crate::components::auth::HandleGAuth;

#[component]
pub fn GoogleAuth() -> impl IntoView {
    view! {
        <HandleGAuth/>
    }
}
