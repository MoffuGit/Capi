use leptos::prelude::*;

use crate::components::ui::sidebar::{SideBarCollapsible, Sidebar};
use crate::routes::servers::components::sidebar::SideBarOption;
use crate::routes::servers::components::variants::{
    DiscoverSideBar, InboxSideBar, PrivateSideBar, SearchSideBar, ServerSideBar, ServersSideBar,
};

use super::sidebar::{SideBarData, SideBarRoute};

#[component]
pub fn SidebarCollapsible(
    route: Memo<SideBarRoute>,
    option: RwSignal<Option<SideBarOption>>,
    data: Signal<Option<Vec<SideBarData>>>,
) -> impl IntoView {
    view! {
        <Sidebar collapsible=SideBarCollapsible::None class="flex-1 md:flex min-w-[250px] relative">
            {
                move || {
                     option.get().map(|option| {
                        view!{
                            <div class="absolute inset-0 bg-sidebar z-50">
                                {
                                    move || {
                                        match option {
                                            SideBarOption::Search => view!{<SearchSideBar/>}.into_any(),
                                            SideBarOption::Inbox => view!{<InboxSideBar/>}.into_any(),
                                        }
                                    }

                                }
                            </div>
                        }
                    })
                }
            }
            // {
            //     move || match route.get() {
            //         SideBarRoute::Server  => view!{<ServerSideBar data=data/>}.into_any(),
            //         SideBarRoute::Discover  => view!{<DiscoverSideBar/>}.into_any(),
            //         SideBarRoute::Servers  => view!{<ServersSideBar/>}.into_any(),
            //         SideBarRoute::Private  => view!{<PrivateSideBar/>}.into_any(),
            //     }
            //
            // }
        </Sidebar>
    }
}
