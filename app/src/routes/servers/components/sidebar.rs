use api::convex::Query;
use common::convex::Servers;
use leptos::prelude::*;
use leptos_router::components::A;
use serde_json::json;

use crate::components::icons::{GlobeIcon, IconCommand, IconInbox, IconSearch};
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarHeader, SidebarInput, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarRail,
};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::hooks::sycn::SyncSignal;
use crate::routes::user_user;

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <Sidebar collapsible=SideBarCollapsible::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <Sidebar
                collapsible=SideBarCollapsible::None
                class="w-[calc(var(--sidebar-width-icon)+1px)]! border-r"
              >
                <SidebarHeader>
                  <SidebarMenu>
                    <SidebarMenuItem>
                      <SidebarMenuButton size=SidebarMenuButtonSize::Lg  class="md:h-8 md:p-0">
                        <A href="/servers/me" {..} class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                            <IconCommand class="size-4" />
                        </A>
                      </SidebarMenuButton>
                    </SidebarMenuItem>
                  </SidebarMenu>
                </SidebarHeader>
                <SidebarContent>
                  <SidebarGroup>
                    <SidebarGroupContent class="px-1.5 md:px-0">
                        <SidebarMenu>
                              <SidebarMenuItem>
                                <ToolTip>
                                    <ToolTipTrigger>
                                      <SidebarMenuButton
                                        class="px-2.5 md:px-2"
                                      >
                                        <IconSearch/>
                                      </SidebarMenuButton>
                                    </ToolTipTrigger>
                                    <ToolTipContent side_of_set=3.0 arrow=true>
                                        "Search"
                                    </ToolTipContent>
                                </ToolTip>
                            </SidebarMenuItem>
                              <SidebarMenuItem>
                              <SidebarMenuButton
                                class="px-2.5 md:px-2"
                              >
                                <IconInbox/>
                              </SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton
                                          class="px-2.5 md:px-2"
                                        >
                                            <GlobeIcon/>
                                        </SidebarMenuButton>
                              </SidebarMenuItem>
                            <SideBarServers/>
                        </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                    <div/>
                  // <NavUser user={data.user} />
                </SidebarFooter>
            </Sidebar>

            <Sidebar collapsible=SideBarCollapsible::None class="hidden flex-1 md:flex">
                <SidebarHeader class="gap-3.5 border-b p-4">
                <div class="flex w-full items-center justify-between">
                    <div class="text-foreground text-base font-medium">
                    </div>
                    <Label class="flex items-center gap-2 text-sm">
                        <span>Unreads</span>
                        // <Switch class="shadow-none" />
                    </Label>
                </div>
                <SidebarInput {..} placeholder="Type to search..." />
                </SidebarHeader>
                <SidebarContent>
                    <SidebarGroup class="px-0">
                        <SidebarGroupContent>
                            <div/>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
            </Sidebar>
            <SidebarRail/>
        </Sidebar>

    }
}

#[component]
pub fn SideBarServers() -> impl IntoView {
    let user = user_user();
    let servers: SyncSignal<Vec<Servers>> = SyncSignal::new(Query {
        name: "user:getServers".to_string(),
        args: json!({
            "userId": user.id
        }),
    });
    view! {
        <Show when=move || servers.signal.get().is_some()>
            <For
                each=move || servers.signal.get().unwrap()
                key=|server| server.id.clone()
                let:server
            >
                <SidebarMenuItem>
                    <SidebarMenuButton
                      class="px-2.5 md:px-2"
                    >
                        {server.name}
                    </SidebarMenuButton>
              </SidebarMenuItem>
            </For>
        </Show>
    }
}
