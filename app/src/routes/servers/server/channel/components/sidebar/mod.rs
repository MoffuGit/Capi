mod members;
mod roles;

use common::convex::Member;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::primitives::common::Side;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};

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
    view! {
        <Sidebar class="mt-[61px] h-auto" side=Side::Right>
            <SidebarHeader>
                <div/>
            </SidebarHeader>
            <SidebarContent>
                <RolesItems server=server/>
                <Show when=move || online.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <SidebarGroupLabel>
                                "Online"
                            </SidebarGroupLabel>
                            <MembersItems members=online/>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </Show>
                <Show when=move || offline.get().and_then(|res| res.ok()).is_some_and(|members| !members.is_empty())>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <SidebarGroupLabel>
                                "Offline"
                            </SidebarGroupLabel>
                            <MembersItems members=offline/>
                        </SidebarGroupContent>
                    </SidebarGroup>
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
