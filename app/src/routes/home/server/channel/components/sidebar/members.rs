use capi_primitives::menu::MenuAlign;
use capi_primitives::menu::MenuSide;
use common::convex::Member;
use leptos::prelude::*;

use super::card::MemberCard;
use crate::components::ui::avatar::*;
use crate::components::ui::dropwdown::*;
use crate::components::ui::sidebar::*;

#[component]
pub fn MembersItems(members: ReadSignal<Option<Result<Vec<Member>, String>>>) -> impl IntoView {
    view! {
        <SidebarMenu>
            <For
                each=move || members.get().and_then(|res| res.ok()).unwrap_or_default()
                key=|member| member.id.clone()
                let(member)
                children=move |member| {
                    let member = StoredValue::new(member);
                    view!{
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <SidebarMenuItem>
                                    <SidebarMenuButton
                                        size=SidebarMenuButtonSize::Lg
                                        class="active:scale-[.98] duration-150 transition-[scale] data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                                    >
                                        <Avatar class="h-8 w-8 rounded-lg">
                                            <AvatarImage url=member.get_value().image_url/>
                                            <AvatarFallback class="rounded-lg">{member.get_value().name.chars().take(2).collect::<String>()}</AvatarFallback>
                                        </Avatar>
                                        <div class="grid flex-1 text-left text-sm leading-tight">
                                            <span class="truncate font-medium">{member.get_value().name}</span>
                                        </div>
                                    </SidebarMenuButton>

                                </SidebarMenuItem>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent side=MenuSide::Left align=MenuAlign::Start>
                                <MemberCard member=member.get_value()/>
                            </DropdownMenuContent>
                        </DropdownMenu>

                    }
                }
            />
        </SidebarMenu>
    }
}
