mod members;
mod roles;

use api::convex::Query;
use common::convex::{Member, Role};
use leptos::prelude::*;
use serde_json::json;

use crate::components::primitives::common::Side;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};
use crate::hooks::sycn::SyncSignal;

use self::members::MembersItems;
use self::roles::RolesItems;

#[component]
pub fn MembersSideBar(
    server: Memo<Option<String>>,
    member: RwSignal<Option<Option<Member>>>,
) -> impl IntoView {
    let members: SyncSignal<Vec<Member>> = SyncSignal::new(Memo::new(move |_| {
        server.get().map(|server| Query {
            name: "member:getOnlineMembersByRole".to_string(),
            args: json!({
                "server": server
            }),
        })
    }));
    view! {
        <Sidebar class="mt-[61px] h-auto" side=Side::Right>
            <SidebarHeader>
                <div/>
            </SidebarHeader>
            <SidebarContent>
                <RolesItems server=server/>
                <Show when=move || members.signal.get().is_some_and(|members| !members.is_empty())>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <SidebarGroupLabel>
                                "Active"
                            </SidebarGroupLabel>
                            <MembersItems members=members.signal/>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </Show>
            </SidebarContent>
            <Footer member=member/>
        </Sidebar>
    }
}

#[component]
pub fn Footer(member: RwSignal<Option<Option<Member>>>) -> impl IntoView {
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
                                member.get().flatten().map(|member| {
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
