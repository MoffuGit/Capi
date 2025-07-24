use leptos::prelude::*;

use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarContent, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem,
};

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
pub fn SideBar(setting: RwSignal<Settings>) -> impl IntoView {
    let groups = StoredValue::new(vec![Group::new(
        "Account",
        vec![Settings::Account, Settings::Preferences, Settings::Profiles],
    )]);
    view! {
        <Sidebar collapsible=SideBarCollapsible::None class="rounded-l-xl">
            <SidebarContent>
                {
                    move || {
                        groups.get_value().into_iter().map(|group| {
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
                                                                class="select-none"
                                                            >
                                                                {set.view()}
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
