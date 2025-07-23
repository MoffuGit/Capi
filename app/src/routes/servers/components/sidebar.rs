pub use api::server::SideBarData;
use api::server::{preload_server_data, GetServers};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::auth::use_auth;
use crate::components::icons::{IconMic, IconSettings};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::sidebar::{
    SideBarCollapsible as SideBarCollapsibleType, Sidebar, SidebarRail,
};
use crate::routes::servers::components::collapsible::SidebarCollapsible;
use crate::routes::servers::components::icons::SidebarIcons;
use crate::routes::servers::components::settings::DialogUserSettings;
use crate::routes::use_profile;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarRoute {
    Server,
    Discover,
    Servers,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarOption {
    Search,
    Inbox,
}

/// Helper function to derive SideBarRoute from a given path
fn get_route_from_path(path: &str) -> SideBarRoute {
    match path.split('/').nth(2) {
        None | Some("") => SideBarRoute::Servers, // Covers /servers and /servers/
        Some("discover") => SideBarRoute::Discover,
        Some("me") => SideBarRoute::Private,
        _ => SideBarRoute::Server, // Covers /servers/{id} or anything else
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();

    let option: RwSignal<Option<SideBarOption>> = RwSignal::new(None);

    let route = Memo::new(move |_| {
        let path = location.pathname.get();
        get_route_from_path(&path)
    });

    let auth = use_auth().auth();

    let preloaded_data = Resource::new(move || (), move |_| preload_server_data());

    let user = use_profile();

    let open_user_settings = RwSignal::new(false);

    view! {
        <Sidebar collapsible=SideBarCollapsibleType::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <Transition>
                {
                    move || {
                        preloaded_data.and_then(|data| {
                            let data = UseQuery::with_preloaded(move || {
                                auth.get()
                                    .and_then(|res| res.ok())
                                    .flatten()
                                    .map(|auth| GetServers { auth: auth.id })
                            }, data.clone());
                            let data = Signal::derive(move || data.get().and_then(|res| res.ok()));
                            view!{
                                <SidebarIcons data=data option=option/>
                                <SidebarCollapsible data=data route=route option=option/>
                            }
                        })
                    }
                }
            </Transition>
            <div class="bg-background h-8 shadow-md border rounded-lg flex items-center bottom-2 left-2 absolute group-data-[state=collapsed]:w-8 group-data-[state=expanded]:p-2 group-data-[state=expanded]:w-[calc(var(--sidebar-width)-18px)] group-data-[state=expanded]:h-12 transition-all ease-in-out-cubic duration-200 overflow-hidden">
                {
                    move || {
                        user.get().map(|user| {
                            let name = StoredValue::new(user.name);
                            view!{
                                <Avatar class="flex bg-accent aspect-square size-8 items-center justify-center rounded-lg group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                                    <AvatarImage url=user.image_url/>
                                    <AvatarFallback class="rounded-lg select-none bg-transparent">
                                        {name.get_value().chars().next()}
                                    </AvatarFallback>
                                </Avatar>
                                <div class="flex justify-center items-center ml-auto gap-2">
                                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                                        <IconMic/>
                                    </Button>
                                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost on:click=move |_| {
                                        open_user_settings.set(true);
                                    }>
                                        <IconSettings/>
                                    </Button>
                                </div>
                            }
                        })
                    }
                        }
            </div>
            <SidebarRail/>
        </Sidebar>
        <DialogUserSettings open=open_user_settings />
    }
}
