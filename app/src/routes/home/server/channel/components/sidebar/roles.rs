use common::convex::Role;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::Serialize;
use tailwind_fuse::tw_merge;

use crate::components::icons::IconChevronDown;
use crate::components::ui::collapsible::*;
use crate::components::ui::sidebar::*;
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
                    let is_open = RwSignal::new(true);
                    view! {
                        <Show when=move || members.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                        <Collapsible open=is_open>
                            <SidebarGroup>
                                <CollapsibleTrigger>
                                    <SidebarGroupLabel class="px-1 hover:text-sidebar-foreground transition-all select-none cursor-pointer">
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
                                <SidebarGroupContent>
                                    <CollapsiblePanel>
                                        <MembersItems members=members/>
                                    </CollapsiblePanel>
                                </SidebarGroupContent>
                            </SidebarGroup>
                        </Collapsible>
                        </Show>
                    }
                }
            />
        </Show>
    }
}
