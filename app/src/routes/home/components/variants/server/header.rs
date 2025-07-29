use common::convex::Server;
use leptos::prelude::*;

use crate::components::icons::{IconBox, IconChevronDown, IconPlus, IconSettings, IconUsers};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::roles::*;
use crate::components::ui::avatar::*;
use crate::components::ui::dropwdown::*;
use crate::components::ui::sidebar::*;

#[component]
pub fn ServerHeader(server: Memo<Option<Server>>) -> impl IntoView {
    view! {
        {
            move || {
                server.get().map(|server| {
                    let name = StoredValue::new(server.name.clone());
                    let image_url = StoredValue::new(server.image_url.clone());
                    view!{
                        <DropdownMenu>
                            <SidebarHeader class="flex w-full">
                                <DropdownMenuTrigger class="flex w-full">
                                        <SidebarMenu>
                                            <SidebarMenuItem>
                                                <SidebarMenuButton size=SidebarMenuButtonSize::Lg>
                                                    <Avatar class="flex bg-accent aspect-square size-8 items-center justify-center rounded-lg">
                                                        <AvatarImage url=image_url.get_value()/>
                                                        <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                            {name.get_value().chars().next()}
                                                        </AvatarFallback>
                                                    </Avatar>
                                                    <div class="grid flex-1 text-left text-base capitalize">
                                                        <span class="truncate font-semibold">
                                                            {
                                                                name.get_value()
                                                            }
                                                        </span>
                                                    </div>
                                                    <IconChevronDown class="ml-auto"/>
                                                </SidebarMenuButton>
                                            </SidebarMenuItem>
                                        </SidebarMenu>
                                </DropdownMenuTrigger>
                            </SidebarHeader>
                            <DropdownMenuContent side=MenuSide::Bottom align=MenuAlign::Center>
                                <DropdownMenuGroup>
                                    <DropdownMenuLabel>
                                        {name.get_value()}
                                    </DropdownMenuLabel>
                                    <CanManageChannels>
                                        <DropdownMenuItem>
                                            <IconPlus/>
                                            "Create Channel"
                                        </DropdownMenuItem>
                                    </CanManageChannels>
                                    <CanManageCategories>
                                        <DropdownMenuItem>
                                            <IconBox/>
                                            "Create Category"
                                        </DropdownMenuItem>
                                    </CanManageCategories>
                                    <CanCreateInvitation>
                                        <DropdownMenuItem>
                                            <IconUsers/>
                                            "Invite People"
                                        </DropdownMenuItem>
                                    </CanCreateInvitation>
                                    <CanManageServerSettings>
                                        <DropdownMenuItem>
                                            <IconSettings/>
                                            "Settings"
                                        </DropdownMenuItem>
                                    </CanManageServerSettings>
                                </DropdownMenuGroup>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    }
                })
            }

        }
    }
}
