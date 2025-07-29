mod content;
mod sidebar;

use crate::components::ui::avatar::*;
use crate::components::ui::dialog::*;
use crate::components::ui::sidebar::SidebarProvider;
use common::convex::Server;
use leptos::prelude::*;

use self::content::Content;
use self::sidebar::SideBar;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Settings {
    Profile,
    Members,
    Roles,
    Invites,
}

impl Settings {
    pub fn view(&self, server: Server) -> impl IntoView {
        match self {
            Settings::Profile => {
                let name = StoredValue::new(server.name);
                let image_url = StoredValue::new(server.image_url);
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

                }.into_any()
            }
            Settings::Members => view! {
                "Members"
            }
            .into_any(),
            Settings::Roles => view! {
                "Roles"
            }
            .into_any(),
            Settings::Invites => view! {
                "Invites"
            }
            .into_any(),
        }
    }
}

#[component]
pub fn DialogServerSettings(
    open: RwSignal<bool>,
    #[prop(into)] server: Signal<Option<Server>>,
) -> impl IntoView {
    let setting = RwSignal::new(Settings::Profile);
    view! {
        <Dialog open=open>
            <DialogPopup class="max-h-[715px] h-[calc(-100px+100vh)] rounded-xl overflow-hidden p-0 w-[1150px] md:max-w-[700px] lg:max-w-[800px] xl:max-w-[1150px]">
                <SidebarProvider style="--sidebar-width: 240px" main=false class="h-full min-h-full">
                    <SideBar setting=setting server=server/>
                    <Content setting=setting server=server/>
                </SidebarProvider>
            </DialogPopup>
        </Dialog>

    }
}
