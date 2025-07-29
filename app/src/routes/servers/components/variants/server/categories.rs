use api::channel::preload_channels;
use common::convex::{Category, Server};
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::icons::{IconChevronDown, IconPlus};
use crate::components::ui::collapsible::{Collapsible, CollapsiblePanel, CollapsibleTrigger};
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
};
use crate::routes::servers::components::dialogs::create_channel::CreateChannelDialog;
use crate::routes::servers::components::variants::server::channels::ChannelsItems;

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
                let is_open = RwSignal::new(false);
                let category_id = StoredValue::new(category.id.clone());
                let preloaded_channels = Resource::new(
                    move || (server.get(), category_id.get_value()),
                    move |(server, category)| preload_channels(server.map(|server| server.id), Some(category)),
                );
                let open_create_channel = RwSignal::new(false);
                view!{
                    <Collapsible>
                        <SidebarGroup>
                            <CollapsibleTrigger>
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
                            </CollapsibleTrigger>
                            <SidebarGroupAction on:click=move |_| {
                                open_create_channel.set(true);
                            }>
                                <IconPlus class="text-sidebar-foreground/70"/>
                                <span class="sr-only">Add channel</span>
                            </SidebarGroupAction>
                            <Transition>
                                {
                                    move || {
                                        preloaded_channels.and_then(|channels| {
                                            let channels = StoredValue::new(channels.clone());
                                            view!{
                                                <CollapsiblePanel>
                                                    <SidebarGroupContent>
                                                        <ChannelsItems preloaded_channels=channels.get_value() server=server category=category_id.get_value() />
                                                    </SidebarGroupContent>
                                                </CollapsiblePanel>
                                            }
                                        })
                                    }
                                }
                            </Transition>
                        </SidebarGroup>
                    </Collapsible>
                    <CreateChannelDialog categories=categories category=category server=server open=open_create_channel />
                }
            }
        />
    }
}
