use api::convex::Query;
use common::convex::Server;
use leptos::prelude::*;
use serde_json::json;

use crate::components::auth::use_auth;
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
    view! {
        <Show when=move || servers.signal.get().is_some()>
            <For
                each=move || servers.signal.get().unwrap()
                key=|server| server.id.clone()
                let:server
            >
                <SidebarMenuItem>
                    <SidebarMenuButton
                    >
                        {server.name.chars().next()}
                    </SidebarMenuButton>
              </SidebarMenuItem>
            </For>
        </Show>
    }
}
