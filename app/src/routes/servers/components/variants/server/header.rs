use common::convex::Server;
use leptos::prelude::*;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::sidebar::{
    SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};

#[component]
pub fn ServerHeader(server: Memo<Option<Server>>) -> impl IntoView {
    view! {
        {
            move || {
                server.get().map(|server| {
                    let name = StoredValue::new(server.name.clone());
                    let image_url = StoredValue::new(server.image_url.clone());
                    view!{
                        <SidebarHeader class="flex w-full">
                            <SidebarMenu>
                                <SidebarMenuItem>
                                    <SidebarMenuButton size=SidebarMenuButtonSize::Lg>
                                        <Avatar class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                            <AvatarImage url=image_url.get_value()/>
                                            <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                {name.get_value().chars().next()}
                                            </AvatarFallback>
                                        </Avatar>
                                        <div class="grid flex-1 text-left text-base leading-tight">
                                            <span class="truncate font-medium">
                                                {
                                                    name.get_value()
                                                }
                                            </span>
                                        </div>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                            </SidebarMenu>
                        </SidebarHeader>
                    }
                })
            }

        }
    }
}
