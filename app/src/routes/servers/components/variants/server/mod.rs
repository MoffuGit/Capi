mod categories;
mod channels;

use common::convex::Server;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::sidebar::{
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarHeader, SidebarMenu,
    SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
};

use self::categories::CategoriesItems;
use self::channels::ChannelsItems;

#[component]
pub fn ServerSideBar(servers: RwSignal<Option<Vec<Server>>>) -> impl IntoView {
    let params = use_params_map();
    let server = Signal::derive(move || {
        let id = params.get().get("server");
        let servers = servers.get();
        id.and_then(|id| {
            servers.map(|servers| servers.iter().find(|server| server.id == id).cloned())
        })
        .flatten()
    });

    view! {
        <Show when=move || server.get().is_some()>
            {
                move || {
                    server.get().map(|server| {
                        let name = StoredValue::new(server.name.clone());
                        let image_url = StoredValue::new(server.image_url.clone());
                        view!{
                            <SidebarHeader>
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
                <ContextMenu>
                    <SidebarContent>
                        <SidebarGroup>
                            <SidebarGroupContent>
                                <ChannelsItems server=server/>
                                <CategoriesItems server=server/>
                            </SidebarGroupContent>
                        </SidebarGroup>
                        <ContextMenuTrigger class="w-full h-full"/>
                    </SidebarContent>
                    <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                        <ContextMenuItem>
                            "Create Channel"
                        </ContextMenuItem>
                        <ContextMenuItem close_on_click=true>
                            "Create Category"
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
        </Show>
    }
}
