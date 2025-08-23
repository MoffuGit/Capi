mod content;
mod header;

use leptos::prelude::*;
use leptos_router::hooks::use_location;

use capi_ui::tabs::*;

use self::content::Content;
use self::header::Header;

#[component]
pub fn Servers() -> impl IntoView {
    let hash = use_location().hash;
    let tab = RwSignal::new(String::default());
    Effect::new(move |_| {
        if hash.get().contains("discover") {
            tab.set("discover".into())
        } else {
            tab.set("servers".into());
        }
    });
    view! {
        <Tabs class="w-full h-full gap-0" tab=tab>
            <Header />
            <Content />
        </Tabs>
    }
}
