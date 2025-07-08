use api::convex::Query;
use common::convex::{Member, Role};
use leptos::prelude::*;
use serde_json::json;

use crate::components::ui::sidebar::{SidebarGroup, SidebarGroupContent, SidebarGroupLabel};
use crate::hooks::sycn::SyncSignal;
use crate::routes::server::channel::components::sidebar::members::MembersItems;

#[component]
pub fn RolesItems(server: Memo<Option<String>>) -> impl IntoView {
    let roles: SyncSignal<Vec<Role>> = SyncSignal::new(Memo::new(move |_| {
        server.get().map(|server| Query {
            name: "roles:serverRoles".to_string(),
            args: json!({
                "server": server
            }),
        })
    }));
    view! {
        <Show when=move || roles.signal.get().is_some()>
            <For
                each=move || roles.signal.get().unwrap()
                key=|role| role.id.clone()
                children=move |role| {
                    let members: SyncSignal<Vec<Member>> = SyncSignal::new(Memo::new(move |_| {
                        Some(Query {
                            name: "member:getOnlineMembersByRole".to_string(),
                            args: json!({
                                "server": role.server,
                                "role": role.id
                            }),
                        })
                    }));
                    let name = StoredValue::new(role.name);
                    view! {
                        <Show when=move || members.signal.get().is_some_and(|members| !members.is_empty())>
                            <SidebarGroup>
                                <SidebarGroupContent>
                                    <SidebarGroupLabel>
                                        {name.get_value()}
                                    </SidebarGroupLabel>
                                    <MembersItems members=members.signal/>
                                </SidebarGroupContent>
                            </SidebarGroup>
                        </Show>
                    }
                }
            />
        </Show>
    }
}
