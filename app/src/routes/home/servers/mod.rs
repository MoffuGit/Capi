use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServerRoute {
    Servers,
    Discover,
}

#[component]
pub fn Servers() -> impl IntoView {
    let hash = use_location().hash;
    let route = Memo::new(move |_| {
        if hash.get().contains("discover") {
            ServerRoute::Discover
        } else {
            ServerRoute::Servers
        }
    });
    view! {
        <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 p-3 border-b">
        </header>
        <div class="bg-red-500">
        </div>
    }
}
