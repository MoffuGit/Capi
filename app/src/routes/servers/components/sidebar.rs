use api::convex::Query;
use common::convex::{Member, Server};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{
    SideBarCollapsible as SideBarCollapsibleType, Sidebar, SidebarRail,
};
use crate::hooks::sycn::SyncSignal;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SideBarData {
    pub server: Server,
    pub member: Member,
}

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();

    let option: RwSignal<Option<SideBarOption>> = RwSignal::new(None);

    let route = Memo::new(move |_| {
        let path = location.pathname.get();
        get_route_from_path(&path)
    });

    let auth = use_auth();

    let data: SyncSignal<Vec<SideBarData>> = SyncSignal::new(Memo::new(move |_| {
        auth.user.get().flatten().map(|user| Query {
            name: "user:getServers".to_string(),
            args: json!({
                "user": user.id
            }),
        })
    }));

    view! {
        <Sidebar collapsible=SideBarCollapsibleType::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <SidebarIcons data=data.signal option=option/>
            <SidebarCollapsible data=data.signal route=route option=option/>
            <SidebarRail/>
        </Sidebar>
    }
}
