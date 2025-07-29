use common::convex::Role;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::ui::sidebar::{SidebarGroup, SidebarGroupContent, SidebarGroupLabel};
use crate::routes::server::channel::components::sidebar::members::MembersItems;
use crate::routes::server::channel::components::sidebar::GetOnlineMembersByRole;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetServerRoles {
    server: String,
}

impl Query<Vec<Role>> for GetServerRoles {
    fn name(&self) -> String {
        "roles:serverRoles".to_string()
    }
}

#[component]
pub fn RolesItems(server: Memo<Option<String>>) -> impl IntoView {
    let roles = UseQuery::new(move || server.get().map(|server| GetServerRoles { server }));
    view! {
        <Show when=move || roles.get().is_some_and(|res| res.is_ok())>
            <For
                each=move || roles.get().and_then(|res| res.ok()).unwrap()
                key=|role| role.id.clone()
                children=move |role| {
                    let members = UseQuery::new(move || {
                        server
                            .get()
                            .map(|server| GetOnlineMembersByRole { server, role: Some(role.id.clone()) })
                    });
                    let name = StoredValue::new(role.name);
                    view! {
                        <Show when=move || members.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                            <SidebarGroup>
                                <SidebarGroupContent>
                                    <SidebarGroupLabel>
                                        {name.get_value()}
                                    </SidebarGroupLabel>
                                    <MembersItems members=members/>
                                </SidebarGroupContent>
                            </SidebarGroup>
                        </Show>
                    }
                }
            />
        </Show>
    }
}
