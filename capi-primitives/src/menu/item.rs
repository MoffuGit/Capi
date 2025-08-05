use leptos::prelude::*;

use crate::menu::MenuProviderContext;

#[component]
pub fn MenuItem(
    #[prop(optional)] close_on_click: bool,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let MenuProviderContext { open, .. } = use_context().expect("should acces the menu context");
    view! {
        <div
            class=class
            on:click=move |_| {
                if close_on_click {
                    open.set(false)
                }
            }
        >
            {children()}
        </div>
    }
}
