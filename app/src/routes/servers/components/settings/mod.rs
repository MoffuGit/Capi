mod content;
mod sidebar;

use crate::components::icons::{IconCircleUser, IconSettings2};
use crate::components::ui::dialog::{Dialog, DialogPopup};
use crate::components::ui::sidebar::SidebarProvider;
use leptos::prelude::*;

use self::content::Content;
use self::sidebar::SideBar;

#[derive(Debug, Clone, Copy)]
enum Settings {
    Preferences,
    Profiles,
}

impl Settings {
    pub fn view(&self) -> impl IntoView {
        match self {
            Settings::Preferences => view! {
                <IconSettings2 />
                "Preferences"
            }
            .into_any(),
            Settings::Profiles => view! {
                <IconCircleUser />
                "Profiles"
            }
            .into_any(),
        }
    }
}

#[component]
pub fn DialogUserSettings(open: RwSignal<bool>) -> impl IntoView {
    let setting = RwSignal::new(Settings::Preferences);
    view! {
        <Dialog open=open>
            <DialogPopup class="max-h-[715px] h-[calc(-100px+100vh)] max-w-[calc(-100px+80vw)] w-1/2 p-0 rounded-xl">
                <SidebarProvider style="--sidebar-width: 240px" main=false class="h-full min-h-full">
                    <SideBar setting=setting/>
                    <Content setting=setting/>
                </SidebarProvider>
            </DialogPopup>
        </Dialog>

    }
}
