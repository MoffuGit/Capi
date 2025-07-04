use api::convex::Query;
use common::convex::{Channel, Server};
use leptos::prelude::*;
use serde_json::json;

use crate::components::icons::{IconEllipsis, IconPlus};
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuItem,
};
use crate::hooks::sycn::SyncSignal;

#[component]
pub fn ChannelsItems(
    server: Signal<Option<Server>>,
    #[prop(into, optional)] category: Option<String>,
) -> impl IntoView {
    let channels: SyncSignal<Vec<Channel>> = SyncSignal::new(Memo::new(move |_| {
        server.get().map(|server| {
            let args = if let Some(category) = &category {
                json!({
                    "server": server.id,
                        "category": category
                })
            } else {
                json!({
                    "server": server.id,
                })
            };
            Query {
                name: "server:getChannels".to_string(),
                args,
            }
        })
    }));
    view! {
        <SidebarMenu>
            <Show when=move || channels.signal.get().is_some()>
                <For
                    each=move || channels.signal.get().unwrap()
                    key=|channel| channel.id.clone()
                    children=move |channel| {
                        view!{
                            <SidebarMenuItem>
                                <SidebarMenuButton /* class="min-w-full" */>
                                    {channel.name}
                                </SidebarMenuButton>
                                <SidebarMenuAction show_on_hover=true>
                                    <IconEllipsis />
                                    <span class="sr-only">More</span>
                                </SidebarMenuAction>
                            </SidebarMenuItem>
                        }
                    }
                />
            </Show>
        </SidebarMenu>
    }
}
