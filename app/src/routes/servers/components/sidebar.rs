pub use api::server::SideBarData;
use api::server::{preload_server_data, GetServers};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{
    SideBarCollapsible as SideBarCollapsibleType, Sidebar, SidebarRail,
};
use crate::routes::servers::components::collapsible::SidebarCollapsible;
use crate::routes::servers::components::icons::SidebarIcons;

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
            <SidebarRail/>
        </Sidebar>
    }
}
