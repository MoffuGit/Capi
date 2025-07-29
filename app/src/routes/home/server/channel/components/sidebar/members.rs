use common::convex::Member;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{
    SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};

#[component]
pub fn MembersItems(members: ReadSignal<Option<Result<Vec<Member>, String>>>) -> impl IntoView {
    view! {
        <SidebarMenu>
            {
                move || {
                    members.get().and_then(|res| res.ok()).map(|members| {
                        members.into_iter().map(|member| {
                            let name = StoredValue::new(member.name);
                            view!{
                                <SidebarMenuItem>
                                    <SidebarMenuButton
                                        size=SidebarMenuButtonSize::Lg
                                        class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                                    >
                                        <Avatar class="h-8 w-8 rounded-lg">
                                            <AvatarImage url=member.image_url/>
                                            <AvatarFallback class="rounded-lg">{name.get_value().chars().take(2).collect::<String>()}</AvatarFallback>
                                        </Avatar>
                                        <div class="grid flex-1 text-left text-sm leading-tight">
                                            <span class="truncate font-medium">{name.get_value()}</span>
                                        </div>
                                    </SidebarMenuButton>

                                </SidebarMenuItem>
                            }
                        }).collect_view()
                    })
                }

            }
        </SidebarMenu>
    }
}
