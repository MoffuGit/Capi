use api::server::GetServers;
pub use api::server::ServerData;
use api::sidebar::SideBarState;
use convex_client::leptos::UseQuery;
use leptos::prelude::*;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::*;
use crate::routes::home::components::collapsible::SidebarCollapsible;
use crate::routes::home::components::icons::SidebarIcons;
use crate::routes::home::components::profile::Profile;
use crate::routes::SideBarRoute;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarOption {
    Search,
    Inbox,
}

#[component]
pub fn SideBar(
    route: Memo<SideBarRoute>,
    option: RwSignal<Option<SideBarOption>>,
) -> impl IntoView {
    let auth = use_auth().auth;

    let data = UseQuery::new(move || {
        auth.get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|auth| GetServers { auth: auth.id })
    });

    let data = Signal::derive(move || data.get().and_then(|res| res.ok()));

    let state = MaybeProp::derive(move || {
        if option.get().is_none()
            && (route.get() == SideBarRoute::Servers || route.get() == SideBarRoute::Discover)
        {
            Some(SideBarState::Collapsed)
        } else {
            None
        }
    });

    view! {
        <Sidebar state=state collapsible=SideBarCollapsible::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <SidebarIcons data=data option=option/>
            <SidebarCollapsible data=data route=route option=option/>
            <Profile/>
            <SidebarRail/>
        </Sidebar>
    }
}
