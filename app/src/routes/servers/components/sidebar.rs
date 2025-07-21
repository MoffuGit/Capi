use common::convex::{Member, Server};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{
    SideBarCollapsible as SideBarCollapsibleType, Sidebar, SidebarRail,
};
use crate::routes::servers::components::collapsible::SidebarCollapsible;
use crate::routes::servers::components::icons::SidebarIcons;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SideBarData {
    pub server: Server,
    pub member: Member,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetServers {
    user: String,
}

impl Query<Vec<SideBarData>> for GetServers {
    fn name(&self) -> String {
        "user:getServers".into()
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

    let user = use_profile();

    let data = UseQuery::new(move || user.get().map(|user| GetServers { user: user.id }));
    let data = Signal::derive(move || data.get().and_then(|res| res.ok()));

    view! {
        <Sidebar collapsible=SideBarCollapsibleType::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <SidebarIcons data=data option=option/>
            <SidebarCollapsible data=data route=route option=option/>
            <SidebarRail/>
        </Sidebar>
    }
}
