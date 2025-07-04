use api::server::CreateServer;
use leptos::prelude::*;
use leptos_router::components::A;

use super::navbar::Navbar;
use crate::components::icons::{
    IconCirclePlus, IconCommand, IconCompass, IconGlobe, IconInbox, IconSearch,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::dialog::{
    Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPopup, DialogTitle, DialogTrigger,
};
use crate::components::ui::dropwdown::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarHeader, SidebarInput, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarRail,
};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::routes::servers::components::servers::ServersItems;

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
                                <ToolTip>
                                    <ToolTipTrigger>
                                      <SidebarMenuButton
                                        class="px-2.5 md:px-2"
                                      >
                                        <IconInbox/>
                                      </SidebarMenuButton>
                                    </ToolTipTrigger>
                                    <ToolTipContent side_of_set=3.0 arrow=true>
                                        "Inbox"
                                    </ToolTipContent>
                                </ToolTip>
                                    </SidebarMenuItem>
                            <ServerMenu/>
                            <ServersItems/>
                        </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                    <Navbar/>
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
pub fn ServerMenu() -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let create_server: ServerAction<CreateServer> = ServerAction::new();
    let pending = create_server.pending();
    view! {
    <Dialog>
        <SidebarMenuItem>
            <DropdownMenu>
                <A href="/servers">
                    <DropdownMenuTrigger>
                        <ToolTip>
                            <ToolTipTrigger>
                            <SidebarMenuButton
                              class="px-2.5 md:px-2"
                            >
                                <IconGlobe/>
                            </SidebarMenuButton>
                            </ToolTipTrigger>
                            <ToolTipContent arrow=true>
                                "Servers"
                            </ToolTipContent>
                        </ToolTip>
                    </DropdownMenuTrigger>
                </A>
                <DropdownMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    <DialogTrigger as_child=true>
                        <DropdownMenuItem>
                            <IconCirclePlus/>
                            "Create"
                        </DropdownMenuItem>
                    </DialogTrigger>
                    <A href="/servers/discover">
                        <DropdownMenuItem close_on_click=true>
                            <IconCompass />
                            "Search"
                        </DropdownMenuItem>
                    </A>
                </DropdownMenuContent>
            </DropdownMenu>
        </SidebarMenuItem>
        <DialogPopup>
            <DialogHeader>
                <DialogTitle >"Create New Server"</DialogTitle>
                <DialogDescription>
                    "Give your new server a personality with a name and an icon. You can always change it later."
                </DialogDescription>
            </DialogHeader>
                <div class="grid gap-2">
                    <Label {..} for="server-name">Server Name</Label>
                    <Input
                        {..}
                        id="server-name"
                        type="text"
                        placeholder="My Awesome Server"
                        required=true
                        value=name
                        on:input=move |ev| set_name(event_target_value(&ev))
                    />
                </div>
                // { move || if pending() { view! { <p>"Creating server..."</p> }.into_view() } else { Fragment::new(vec![]).into_view() } }
                // { move || if let Some(Err(e)) = error() {
                //     view! { <p class="text-red-500">{ format!("Error: {}", e) }</p> }.into_view()
                // } else if let Some(Ok(())) = value() {
                //     view! { <p class="text-green-500">"Server created successfully!"</p> }.into_view()
                // } else {
                //     Fragment::new(vec![]).into_view()
                // }}
            <DialogFooter>
                <button
                    on:click=move |_| {
                        if !name.get().is_empty() {
                            create_server.dispatch(CreateServer { name: name.get() });
                        }
                    }
                    disabled=move || pending.get()
                    class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
                >
                    "Create"
                </button>
            </DialogFooter>
        </DialogPopup>
    </Dialog>
    }
}
