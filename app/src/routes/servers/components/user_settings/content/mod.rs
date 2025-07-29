mod preferences;
mod profiles;

use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::ui::sidebar::SidebarInset;

use self::preferences::Preferences;
use self::profiles::Profiles;

use super::Settings;

#[component]
pub fn Content(setting: RwSignal<Settings>) -> impl IntoView {
    view! {
        <SidebarInset class="rounded-r-xl py-9 px-12">
            {
                move || {
                    match setting.get() {
                        Settings::Account => view!{}.into_any(),
                        Settings::Preferences => view!{<Preferences/>}.into_any(),
                        Settings::Profiles => view!{<Profiles/>}.into_any(),
                    }
                }
            }
        </SidebarInset>
    }
}

#[component]
pub fn Title(children: Children) -> impl IntoView {
    view! {
        <div class="border-b mb-4 mt-0 pb-3 text-base font-medium">
            {children()}
        </div>
    }
}

#[component]
pub fn Setting(children: Children, #[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    view! {
        <div class=move || tw_merge!("flex items-center justify-between my-2", class.get())>
            {children()}
        </div>
    }
}

#[component]
pub fn SettingData(children: Children) -> impl IntoView {
    view! {
        <div
            class="flex flex-col mr-[5%] w-3/4 gap-1.5"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SettingTitle(children: Children) -> impl IntoView {
    view! {
        <div class="font-normal text-sm">
        {children()}
        </div>
    }
}

#[component]
pub fn SettingDescription(children: Children) -> impl IntoView {
    view! {
        <div class="text-muted-foreground font-normal text-xs">
        {children()}
        </div>
    }
}

#[component]
pub fn SettingAction(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <div class=class>
            {children()}
        </div>
    }
}
