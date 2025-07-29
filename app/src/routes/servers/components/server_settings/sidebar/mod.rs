use common::convex::Server;
use leptos::prelude::*;

use crate::components::ui::sidebar::*;

use super::Settings;

#[derive(Debug, Clone)]
pub struct Group {
    label: String,
    items: Vec<Settings>,
}

impl Group {
    pub fn new(label: &str, items: Vec<Settings>) -> Self {
        Group {
            label: label.into(),
            items,
        }
    }
}

#[component]
pub fn SideBar(setting: RwSignal<Settings>, server: Signal<Option<Server>>) -> impl IntoView {
    let groups = Signal::derive(move || {
        vec![Group::new(
            "Server",
            vec![
                Settings::Profile,
                Settings::Members,
                Settings::Roles,
                Settings::Invites,
            ],
        )]
    });
    view! {
        <Sidebar collapsible=SideBarCollapsible::None class="rounded-l-xl">
            <SidebarContent>
                {
                    move || {
                        groups.get().into_iter().map(|group| {
                            view!{
                                <SidebarGroup>
                                    <SidebarGroupLabel>
                                        {group.label.clone()}
                                    </SidebarGroupLabel>
                                    <SidebarGroupContent>
                                        <SidebarMenu>
                                            {
                                                group.items.into_iter().map(|set| {
                                                    view!{
                                                        <SidebarMenuItem>
                                                            <SidebarMenuButton on:click=move |_| {
                                                                    setting.set(set)
                                                                }
                                                                is_active=Signal::derive(move || {
                                                                    setting.get() == set
                                                                })
                                                                class="select-none"
                                                            >
                                                                {
                                                                    move || {
                                                                        server.get().map(|server| {
                                                                            set.view(server)
                                                                        })
                                                                    }
                                                                }
                                                            </SidebarMenuButton>
                                                        </SidebarMenuItem>
                                                    }
                                                }).collect_view()
                                            }
                                            <div/>
                                        </SidebarMenu>
                                    </SidebarGroupContent>
                                </SidebarGroup>
                            }
                        }).collect_view()
                    }
                }
            </SidebarContent>
        </Sidebar>

    }
}
