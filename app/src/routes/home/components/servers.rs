use api::category::GetCategories;
use common::convex::{Member, Role, Server};
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::roles::RolesProvider;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::context::{ContextMenu, ContextMenuTrigger};
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuItem};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::routes::home::components::variants::ServerContextMenuData;

use super::sidebar::ServerData;

#[component]
pub fn ServersItems(
    data: Signal<Option<Vec<ServerData>>>,
    set_option: Callback<()>,
) -> impl IntoView {
    view! {
        <For
            each=move || data.get().unwrap_or_default()
            key=|data| data.server.id.clone()
            let(
                ServerData {
                    server,
                    member,
                    roles
                }

            )
        >
            <ServerItem server=server member=member roles=roles set_option=set_option/>
        </For>
    }
}

#[component]
pub fn ServerItem(
    server: Server,
    member: Member,
    roles: Vec<Role>,
    set_option: Callback<()>,
) -> impl IntoView {
    let params = use_params_map();
    let server = RwSignal::new(server);
    let member = RwSignal::new(member);
    let roles = RwSignal::new(Some(roles));
    let categories = UseQuery::new(move || {
        Some(GetCategories {
            server: server.get().id,
        })
    });
    let categories = Signal::derive(move || categories.get().and_then(|res| res.ok()));
    let is_active = Signal::derive(move || {
        params
            .get()
            .get("server")
            .is_some_and(|s| s == server.get().id)
    });
    let href = move || {
        if let Some(last) = member.get().last_visited_channel {
            format!("/servers/{}/{}", server.get().id, last)
        } else {
            format!("/servers/{}", server.get().id)
        }
    };
    view! {
        <ContextMenu>
            <RolesProvider roles=roles>
                <SidebarMenuItem>
                    <ContextMenuTrigger pointer=false >
                        <A
                            href=move || href()
                            {..}
                            on:click=move |_| {
                                set_option.run(())
                            }
                        >
                            <ToolTip>
                                <ToolTipTrigger>
                                    <SidebarMenuButton
                                        is_active=is_active
                                        size=crate::components::ui::sidebar::SidebarMenuButtonSize::Sm
                                        class="md:h-8 md:p-0 flex items-center justify-center"
                                    >
                                        <Avatar class="h-8 w-8 rounded-lg">
                                            <AvatarImage url=MaybeProp::derive(move || server.get().image_url)/>
                                            <AvatarFallback class="rounded-lg text-sidebar-foreground/70 select-none bg-transparent">
                                                {move || server.get().name.chars().next()}
                                            </AvatarFallback>
                                        </Avatar>
                                    </SidebarMenuButton>
                                </ToolTipTrigger>
                                <ToolTipContent side_of_set=3.0 >
                                    {move || server.get().name}
                                </ToolTipContent>
                            </ToolTip>
                        </A>
                    </ContextMenuTrigger>
                    <ServerContextMenuData
                        categories=categories
                        server=Signal::derive(move || Some(server.get()))
                        member=Signal::derive(move || Some(member.get()))
                    />
                </SidebarMenuItem>
            </RolesProvider>
        </ContextMenu>
    }
}
