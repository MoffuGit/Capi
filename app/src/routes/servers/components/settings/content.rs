use leptos::prelude::*;

use crate::components::ui::sidebar::SidebarInset;

use super::Settings;

#[component]
pub fn Content(setting: RwSignal<Settings>) -> impl IntoView {
    view! {
        <SidebarInset class="rounded-r-xl py-9 px-12">
            {move || {
                format!("{:?}", setting.get())
            }}
        </SidebarInset>
    }
}
