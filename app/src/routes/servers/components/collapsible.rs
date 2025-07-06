use leptos::prelude::*;

use crate::components::ui::sidebar::{SideBarCollapsible, Sidebar};
use crate::routes::servers::components::sidebar::SideBarOption;
use crate::routes::servers::components::variants::{
    DiscoverSideBar, InboxSideBar, PrivateSideBar, SearchSideBar, ServerSideBar, ServersSideBar,
};

use super::sidebar::{SideBarData, SideBarRoute};

#[component]
pub fn SidebarCollapsible(
    selected: Memo<SideBarRoute>,
    data: RwSignal<Option<Vec<SideBarData>>>,
) -> impl IntoView {
    view! {
        <Sidebar collapsible=SideBarCollapsible::None class="flex-1 md:flex min-w-[250px]">
            {
                move || match selected.get() {
                    SideBarRoute::Server  => view!{<ServerSideBar data=data/>}.into_any(),
                    SideBarRoute::Discover  => view!{<DiscoverSideBar/>}.into_any(),
                    SideBarRoute::Servers  => view!{<ServersSideBar/>}.into_any(),
                    SideBarRoute::Private  => view!{<PrivateSideBar/>}.into_any(),
                    SideBarRoute::Option(SideBarOption::Inbox) => view!{<InboxSideBar/>}.into_any(),
                    SideBarRoute::Option(SideBarOption::Search) => view!{<SearchSideBar/>}.into_any(),
                    SideBarRoute::None => ().into_any()
                }

            }
        </Sidebar>
    }
}
