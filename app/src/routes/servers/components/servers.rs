use api::convex::Query;
use common::convex::Server;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde_json::json;

use crate::components::auth::use_auth;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuItem};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::hooks::sycn::SyncSignal;

#[component]
pub fn ServersItems() -> impl IntoView {
    let auth = use_auth();
    let servers: SyncSignal<Vec<Server>> = SyncSignal::new(Memo::new(move |_| {
        auth.user.signal.get().flatten().map(|user| Query {
            name: "user:getServers".to_string(),
            args: json!({
                "user": user.id
            }),
        })
    }));
    let params = use_params_map();
    view! {
        <Show when=move || servers.signal.get().is_some()>
            <For
                each=move || servers.signal.get().unwrap()
                key=|server| server.id.clone()
                children=move |server| {
                    let name = StoredValue::new(server.name.clone());
                    let id = StoredValue::new(server.id);
                    let is_active = Signal::derive(move || {
                        params
                            .get()
                            .get("server")
                            .is_some_and(|s| s == id.get_value())
                    });
                    let image_url = StoredValue::new(server.image_url);
                    view!{
                        <SidebarMenuItem>
                            <ToolTip>
                                <ToolTipTrigger>
                                    <A href=format!("/servers/{}", id.get_value())>
                                        <SidebarMenuButton
                                            is_active=is_active
                                            size=crate::components::ui::sidebar::SidebarMenuButtonSize::Lg
                                            class="md:h-8 md:p-0"
                                        >
                                            <Avatar class="h-8 w-8 rounded-lg">
                                                <AvatarImage url=image_url.get_value()/>
                                                <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                    {name.get_value().chars().next()}
                                                </AvatarFallback>
                                            </Avatar>
                                        </SidebarMenuButton>
                                    </A>
                                </ToolTipTrigger>
                                <ToolTipContent side_of_set=3.0 arrow=true>
                                    {name.get_value()}
                                </ToolTipContent>
                            </ToolTip>
                      </SidebarMenuItem>
                    }

                }
            />
        </Show>
    }
}
