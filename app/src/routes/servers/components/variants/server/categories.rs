use api::category::{preload_categories, GetCategories};
use common::convex::Server;
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::icons::{IconChevronDown, IconPlus};
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
};
use crate::routes::servers::components::variants::server::channels::ChannelsItems;

#[component]
pub fn CategoriesItems(server: Memo<Option<Server>>) -> impl IntoView {
    let preloaded_categories = Resource::new(
        move || (server.get()),
        move |server| preload_categories(server.map(|server| server.id)),
    );
    view! {
        <Transition>
            {
                move || {
                    preloaded_categories.and_then(|categories| {
                        let categories = UseQuery::with_preloaded(move || {
                            server
                                .get()
                                .map(|server| GetCategories { server: server.id })
                        }, categories.clone());
                        let categories = Signal::derive(move || categories.get().and_then(|res| res.ok()));
                        let is_open = RwSignal::new(false);
                        view!{
                            <Show when=move || categories.get().is_some()>
                                <For
                                    each=move || categories.get().unwrap()
                                    key=|category| category.id.clone()
                                    children=move |category| {
                                        let name = StoredValue::new(category.name);
                                        view!{
                                            <SidebarGroup>
                                                <SidebarGroupLabel
                                                    class="px-1 hover:text-sidebar-foreground transition-all select-none cursor-pointer"
                                                    on:click=move |_| {
                                                        is_open.update(|open| *open = !*open);
                                                    }
                                                >
                                                    <IconChevronDown class=Signal::derive(
                                                        move || {
                                                            tw_merge!("mr-1", if is_open.get() {
                                                                    "rotate-0"
                                                                } else {
                                                                    "-rotate-90"
                                                                },
                                                                "transition-transform ease-in-out-quad duration-150"
                                                            )
                                                        }
                                                    )/>
                                                    {name.get_value()}
                                                </SidebarGroupLabel>
                                                <SidebarGroupAction>
                                                    <IconPlus class="text-sidebar-foreground/70"/>
                                                    <span class="sr-only">Add channel</span>
                                                </SidebarGroupAction>
                                                <SidebarGroupContent>
                                                    <ChannelsItems server=server category=category.id />
                                                </SidebarGroupContent>
                                            </SidebarGroup>
                                        }
                                    }
                                />
                            </Show>

                        }
                    })
                }
            }
        </Transition>
    }
}
