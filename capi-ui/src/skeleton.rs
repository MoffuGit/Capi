use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Skeleton(#[prop(into)] class: String) -> impl IntoView {
    view! {
         <div
              data-slot="skeleton"
              class=tw_merge!("bg-accent animate-pulse rounded-md", class)
        />
    }
}
