pub mod card;
mod members;
mod roles;

use common::convex::Member;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::Serialize;
use tailwind_fuse::tw_merge;

use icons::IconChevronDown;
use crate::components::ui::avatar::*;
use crate::components::ui::collapsible::*;
use crate::components::ui::sidebar::*;
use capi_primitives::common::Side;

use self::members::MembersItems;
use self::roles::RolesItems;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetOnlineMembersByRole {
    server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
}

impl Query<Vec<Member>> for GetOnlineMembersByRole {
    fn name(&self) -> String {
        "member:getOnlineMembersByRole".to_string()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetOfflineMembers {
    server: String,
}

impl Query<Vec<Member>> for GetOfflineMembers {
    fn name(&self) -> String {
        "member:getOfflineMembers".to_string()
    }
}

#[component]
pub fn MembersSideBar(
    server: Memo<Option<String>>,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let online = UseQuery::new(move || {
        server
            .get()
            .map(|server| GetOnlineMembersByRole { server, role: None })
    });
    let offline = UseQuery::new(move || server.get().map(|server| GetOfflineMembers { server }));
    let is_offline_open = RwSignal::new(true);
    let is_online_open = RwSignal::new(true);
    view! {
        <Sidebar class="mt-[54px] h-auto" side=Side::Right>
            <SidebarContent>
                <RolesItems server=server/>
                <Show when=move || online.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                    <Collapsible open=is_online_open>
                        <SidebarGroup>
                            <CollapsibleTrigger>
                                <SidebarGroupLabel class="px-1 hover:text-sidebar-foreground transition-all select-none cursor-pointer">
                                    <IconChevronDown class=Signal::derive(
                                        move || {
                                            tw_merge!("mr-1", if is_online_open.get() {
                                                    "rotate-0"
                                                } else {
                                                    "-rotate-90"
                                                },
                                                "transition-transform ease-in-out-quad duration-150"
                                            )
                                        }
                                    )/>
                                    "Online"
                                </SidebarGroupLabel>
                            </CollapsibleTrigger>
                            <SidebarGroupContent>
                                <CollapsiblePanel>
                                    <MembersItems members=online/>
                                </CollapsiblePanel>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </Collapsible>
                </Show>
                <Show when=move || offline.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                    <Collapsible open=is_offline_open>
                        <SidebarGroup>
                            <CollapsibleTrigger>
                                <SidebarGroupLabel class="px-1 hover:text-sidebar-foreground transition-all select-none cursor-pointer">
                                    <IconChevronDown class=Signal::derive(
                                        move || {
                                            tw_merge!("mr-1", if is_offline_open.get() {
                                                    "rotate-0"
                                                } else {
                                                    "-rotate-90"
                                                },
                                                "transition-transform ease-in-out-quad duration-150"
                                            )
                                        }
                                    )/>
                                    "Offline"
                                </SidebarGroupLabel>
                            </CollapsibleTrigger>
                            <SidebarGroupContent>
                                <CollapsiblePanel>
                                    <MembersItems members=offline/>
                                </CollapsiblePanel>
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </Collapsible>
                </Show>
            </SidebarContent>
            <Footer member=member/>
        </Sidebar>
    }
}

#[component]
pub fn Footer(member: Signal<Option<Member>>) -> impl IntoView {
    view! {
        <SidebarFooter>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton
                        size=SidebarMenuButtonSize::Lg
                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                    >
                        {
                            move || {
                                member.get().map(|member| {
                                    let name = StoredValue::new(member.name);
                                    view!{
                                        <Avatar class="h-8 w-8 rounded-lg">
                                            <AvatarImage url=member.image_url/>
                                            <AvatarFallback class="rounded-lg">{name.get_value().chars().take(2).collect::<String>()}</AvatarFallback>
                                        </Avatar>
                                        <div class="grid flex-1 text-left text-sm leading-tight">
                                            <span class="truncate font-medium">{name.get_value()}</span>
                                        </div>
                                    }
                                })
                            }
                        }
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarFooter>
    }
}
