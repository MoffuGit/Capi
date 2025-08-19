use api::channel::GetChannels;
use common::convex::{Category, Server};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::roles::CanManageCategories;
use crate::components::ui::collapsible::*;
use crate::components::ui::sidebar::*;
use crate::routes::home::components::dialogs::create_channel::CreateChannelDialog;
use crate::routes::home::components::variants::server::channels::ChannelsItems;
use icons::{IconChevronDown, IconPlus};

#[component]
pub fn CategoriesItems(
    server: Memo<Option<Server>>,
    categories: Signal<Option<Vec<Category>>>,
) -> impl IntoView {
    view! {
        <For
            each=move || categories.get().unwrap_or_default()
            key=|category| category.id.clone()
            children=move |category| {
                let name = StoredValue::new(category.name.clone());
                let is_open = RwSignal::new(true);
                let category_id = StoredValue::new(category.id.clone());
                let open_create_channel = RwSignal::new(false);
                let channels = UseQuery::new(move || {
                    server.get().map(|server| GetChannels {
                        server: server.id,
                        category: Some(category_id.get_value()),
                    })
                });
                view!{
                    <Collapsible open=is_open>
                        <SidebarGroup>
                            <CollapsibleTrigger>
                                <SidebarGroupLabel
                                    class="px-1 hover:text-sidebar-foreground transition-all select-none cursor-pointer group-data-[collapsible=icon]:mt-0"
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
                            </CollapsibleTrigger>
                            <CanManageCategories>
                                <SidebarGroupAction on:click=move |_| {
                                    open_create_channel.set(true);
                                }>
                                    <IconPlus class="text-sidebar-foreground/70"/>
                                    <span class="sr-only">Add channel</span>
                                </SidebarGroupAction>
                            </CanManageCategories>
                            <SidebarGroupContent>
                                <CollapsiblePanel>
                                    <ChannelsItems channels=channels />
                                </CollapsiblePanel>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </Collapsible>
                    <CreateChannelDialog categories=categories category=category server=server open=open_create_channel />
                }
            }
        />
    }
}
