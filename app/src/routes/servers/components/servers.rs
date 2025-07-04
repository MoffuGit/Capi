use api::convex::Query;
use common::convex::Server;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use serde_json::json;

use crate::components::auth::use_auth;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuItem};
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
    let is_active = move |id: &String| {
        params
            .get()
            .get("server")
            .is_some_and(|server| &server == id)
    };
    view! {
        <Show when=move || servers.signal.get().is_some()>
            <For
                each=move || servers.signal.get().unwrap()
                key=|server| server.id.clone()
                let:server
            >
                <SidebarMenuItem>
                    <A href=format!("/servers/{}", server.id)>
                        <SidebarMenuButton
                            is_active=Signal::derive(move || is_active(&server.id))
                            size=crate::components::ui::sidebar::SidebarMenuButtonSize::Lg
                            class="md:h-8 md:p-0"
                        >
                            <Avatar class="h-8 w-8 rounded-lg">
                                <AvatarImage url=server.image_url/>
                                <AvatarFallback class="rounded-lg select-none bg-transparent">
                                    {server.name.chars().next()}
                                </AvatarFallback>
                            </Avatar>
                        </SidebarMenuButton>
                    </A>
              </SidebarMenuItem>
            </For>
        </Show>
    }
}
