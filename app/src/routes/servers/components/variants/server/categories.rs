use common::convex::{Category, Server};
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::icons::{IconChevronDown, IconPlus};
use crate::components::ui::collapsible::{Collapsible, CollapsiblePanel, CollapsibleTrigger};
use crate::components::ui::sidebar::{
    SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
};
use crate::routes::servers::components::variants::server::channels::ChannelsItems;

#[component]
pub fn CategoriesItems(
    server: Memo<Option<Server>>,
    categories: Signal<Option<Vec<Category>>>,
) -> impl IntoView {
    view! {
        <Show when=move || categories.get().is_some()>
            <For
                each=move || categories.get().unwrap()
                key=|category| category.id.clone()
                children=move |category| {
                    let name = StoredValue::new(category.name);
                    let is_open = RwSignal::new(false);
                    let category_id = StoredValue::new(category.id);
                    view!{
                        <SidebarGroup>
                            <Collapsible>
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
                                <SidebarGroupAction>
                                    <IconPlus class="text-sidebar-foreground/70"/>
                                    <span class="sr-only">Add channel</span>
                                </SidebarGroupAction>
                                <CollapsiblePanel>
                                    <SidebarGroupContent>
                                        <ChannelsItems server=server category=category_id.get_value() />
                                    </SidebarGroupContent>
                                </CollapsiblePanel>
                            </Collapsible>
                        </SidebarGroup>
                    }
                }
            />
        </Show>

    }
}
