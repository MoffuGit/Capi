use convex_client::leptos::UseMutation;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::roles::RolesProvider;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::context::{ContextMenu, ContextMenuTrigger};
use crate::components::ui::sidebar::{SidebarMenuButton, SidebarMenuItem};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::routes::servers::components::variants::{
    CreateCategory, CreateChannel, ServerContextMenuData,
};

use super::sidebar::SideBarData;

#[component]
pub fn ServersItems(
    data: Signal<Option<Vec<SideBarData>>>,
    set_option: Callback<()>,
) -> impl IntoView {
    let params = use_params_map();
    let create_channel = UseMutation::new::<CreateChannel>();
    let create_category = UseMutation::new::<CreateCategory>();
    view! {
        <For
            each=move || data.get().unwrap_or_default()
            key=|data| data.server.id.clone()
            children=move |data| {
                let server = Signal::derive(move || data.server.clone());
                let member = Signal::derive(move || data.member.clone());
                let is_active = Signal::derive(move || {
                    params
                        .get()
                        .get("server")
                        .is_some_and(|s| s == server.get().id)
                });
                let href = move || if let Some(last) = member.get().last_visited_channel {
                    format!("/servers/{}/{}", server.get().id, last)
                } else {
                    format!("/servers/{}", server.get().id)
                };
                view!{
                    <RolesProvider roles=data.roles>
                        <SidebarMenuItem>
                            <ContextMenu>
                                <A
                                    href=move || href()
                                    {..}
                                    on:click=move |_| {
                                        set_option.run(())
                                    }
                                >
                                    <ContextMenuTrigger pointer=false >
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
                                    </ContextMenuTrigger>
                                    <ServerContextMenuData
                                        server=server
                                        member=member
                                        create_channel=create_channel
                                        create_category=create_category
                                    />
                                </A>
                            </ContextMenu>
                      </SidebarMenuItem>
                    </RolesProvider>

                }
            }
        />
    }
}
