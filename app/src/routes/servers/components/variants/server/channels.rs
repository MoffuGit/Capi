use api::convex::Query;
use common::convex::{Channel, Server};
use leptos::prelude::*;
use leptos_dom::log;
use leptos_router::components::A;
use serde_json::json;

use crate::components::icons::{IconEllipsis, IconPlus};
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuItem,
};
use crate::hooks::sycn::SyncSignal;

#[component]
pub fn ChannelsItems(
    server: Memo<Option<Server>>, // Changed from StoredValue<Server> to Memo<Option<Server>>
    #[prop(into, optional)] category: Option<String>,
) -> impl IntoView {
    let channels: SyncSignal<Vec<Channel>> = SyncSignal::new(Memo::new(move |_| {
        server.get().map(|s| {
            // `s` is `Option<Server>`
            let args = if let Some(category) = &category {
                json!({
                    "server": s.id,
                        "category": category
                })
            } else {
                json!({
                    "server": s.id,
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
            {
                move || {
                        channels.signal.get().map(|channels| {
                            channels.into_iter().map(|channel|
                                view!{
                                    <A href=move || format!("/servers/{}/{}", channel.server,  channel.id)>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton /* class="min-w-full" */>
                                                {channel.name}
                                            </SidebarMenuButton>
                                            <SidebarMenuAction show_on_hover=true>
                                                <IconEllipsis />
                                                <span class="sr-only">More</span>
                                            </SidebarMenuAction>
                                        </SidebarMenuItem>
                                    </A>
                                }
                            ).collect_view()
                        })
                }
            }
        </SidebarMenu>
    }
}
