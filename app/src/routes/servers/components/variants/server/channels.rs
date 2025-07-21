use common::convex::{Channel, Server};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use leptos_router::components::A;
use serde::Serialize;

use crate::components::icons::IconEllipsis;
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuItem,
};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GetChannels {
    server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
}

impl Query<Vec<Channel>> for GetChannels {
    fn name(&self) -> String {
        "server:getChannels".to_string()
    }
}

#[component]
pub fn ChannelsItems(
    server: Memo<Option<Server>>,
    #[prop(into, optional)] category: Option<String>,
) -> impl IntoView {
    let channels = UseQuery::new(move || {
        server.get().map(|server| GetChannels {
            server: server.id,
            category: category.clone(),
        })
    });
    view! {
        <SidebarMenu>
            {
                move || {
                        channels.get().and_then(|channels| {
                                channels.ok().map(|channels| {
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
                        })
                }
            }
        </SidebarMenu>
    }
}
