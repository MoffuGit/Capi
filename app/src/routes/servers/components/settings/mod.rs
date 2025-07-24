mod content;
mod sidebar;

use crate::components::icons::{IconCircleUser, IconSettings2};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::dialog::{Dialog, DialogPopup};
use crate::components::ui::sidebar::SidebarProvider;
use crate::routes::use_profile;
use leptos::prelude::*;

use self::content::Content;
use self::sidebar::SideBar;

#[derive(Debug, Clone, Copy)]
enum Settings {
    Account,
    Preferences,
    Profiles,
}

impl Settings {
    pub fn view(&self) -> impl IntoView {
        match self {
            Settings::Account => {
                let user = use_profile();

                view! {
                    {
                        move || {
                            user.get().map(|user| {
                                let name = StoredValue::new(user.name);
                                let image_url = StoredValue::new(user.image_url);
                                view!{
                                    <Avatar class="flex bg-accent aspect-square size-4 items-center justify-center rounded-lg group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                                        <AvatarImage url=image_url.get_value()/>
                                        <AvatarFallback class="rounded-lg select-none bg-transparent">
                                            {name.get_value().chars().next()}
                                        </AvatarFallback>
                                    </Avatar>
                                    <span class="text-ellipsis">
                                        {name.get_value()}
                                    </span>
                                }
                            })
                        }
                    }
                }.into_any()
            }
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
            <DialogPopup class="max-h-[715px] h-[calc(-100px+100vh)] rounded-xl overflow-hidden p-0 w-[1150px] md:max-w-[700px] lg:max-w-[800px] xl:max-w-[1150px]">
                <SidebarProvider style="--sidebar-width: 240px" main=false class="h-full min-h-full">
                    <SideBar setting=setting/>
                    <Content setting=setting/>
                </SidebarProvider>
            </DialogPopup>
        </Dialog>

    }
}
