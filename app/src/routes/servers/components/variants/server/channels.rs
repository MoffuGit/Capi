use common::convex::{Channel, Server};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
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
    let location = use_location();
    let path = location.pathname;
    let current_channel = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|channel| channel.to_string())
    });
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
                                    channels.into_iter().map(|channel| {
                                        let name = StoredValue::new(channel.name);
                                        let id = StoredValue::new(channel.id);
                                        view!{
                                            <A href=move || format!("/servers/{}/{}", channel.server,  id.get_value())>
                                                <SidebarMenuItem>
                                                    <SidebarMenuButton
                                                        is_active=Signal::derive(
                                                            move || {
                                                                current_channel.get().is_some_and(|curr| {
                                                                     id.get_value() == curr
                                                                })
                                                            }
                                                        )
                                                        class="group/button">
                                                        <span
                                                            class="text-sidebar-foreground/70 w-full inline-flex flex-col items-start font-normal group-data-[active=true]/button:font-bold group-hover/button:font-bold transition-[font-weight] duration-[150ms] ease-out after:content-[attr(data-text)] after:h-0 after:hidden after:overflow-hidden after:select-none after:pointer-events-none after:font-bold"
                                                             data-text={name.get_value()}
                                                        >
                                                            {name.get_value()}
                                                        </span>
                                                    </SidebarMenuButton>
                                                    <SidebarMenuAction show_on_hover=true>
                                                        <IconEllipsis />
                                                        <span class="sr-only">More</span>
                                                    </SidebarMenuAction>
                                                </SidebarMenuItem>
                                            </A>
                                        }

                                        }
                                    ).collect_view()
                            })
                        })
                }
            }
        </SidebarMenu>
    }
}
