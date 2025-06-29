use leptos::prelude::*;

#[component]
pub fn Skeleton(#[prop(into)] class: String) -> impl IntoView {
    view! {
         <div
              data-slot="skeleton"
              class=format!("bg-accent animate-pulse rounded-md {}", class)
        />
    }
}
