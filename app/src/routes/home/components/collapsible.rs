use crate::routes::home::components::variants::*;
use leptos::prelude::*;

use crate::components::ui::sidebar::{SideBarCollapsible, Sidebar};
use crate::routes::home::components::sidebar::SideBarOption;
use crate::routes::SideBarRoute;

use super::sidebar::SideBarData;

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
            {
                move || match route.get() {
                    SideBarRoute::Private  => view!{<PrivateSideBar/>}.into_any(),
                    _ => view!{
                        <ServerSideBar data=data/>
                    }.into_any()
                }

            }
        </Sidebar>
    }
}
