pub mod channel;
mod components;

use leptos::prelude::*;

use self::components::Header;

#[component]
pub fn Server() -> impl IntoView {
    view! {
        <Header/>
    }
}
