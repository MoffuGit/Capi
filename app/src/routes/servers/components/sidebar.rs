use api::server::CreateServer;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

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

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarRoute {
    Server,
    Discover,
    Servers,
    Search,
    Inbox,
}

/// Helper function to derive SideBarRoute from a given path
fn get_route_from_path(path: &str) -> SideBarRoute {
    match path.split('/').nth(2) {
        None | Some("") => SideBarRoute::Servers, // Covers /servers and /servers/
        Some("discover") => SideBarRoute::Discover,
        _ => SideBarRoute::Server, // Covers /servers/{id} or anything else
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();
    let (active_sidebar_route, set_active_sidebar_route) = signal(SideBarRoute::Servers);
    let (mounted, set_mounted) = signal(false);

    // Effect to update sidebar_state when the URL path changes
    // This handles actual navigation to different URLs.
    Effect::new(move |_| {
        let path = location.pathname.get();
        let new_state = get_route_from_path(&path);
        set_active_sidebar_route(new_state);
    });

    Effect::new(move |_| {
        set_mounted(true);
    });

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
                        <A href="/servers/me"
                           on:click=move |_| set_active_sidebar_route(get_route_from_path(&location.pathname.get_untracked()))
                            {..}
                           class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
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
                                        on:click=move |_| set_active_sidebar_route(SideBarRoute::Search) // Manual override
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
                                        on:click=move |_| set_active_sidebar_route(SideBarRoute::Inbox) // Manual override
                                      >
                                        <IconInbox/>
                                      </SidebarMenuButton>
                                    </ToolTipTrigger>
                                    <ToolTipContent side_of_set=3.0 arrow=true>
                                        "Inbox"
                                    </ToolTipContent>
                                </ToolTip>
                                    </SidebarMenuItem>
                            <ServerMenu set_active_sidebar_route=set_active_sidebar_route/>
                            <ServersItems/>
                        </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                    <Navbar/>
                </SidebarFooter>
            </Sidebar>

            <Show when=move || mounted()>
                <Sidebar collapsible=SideBarCollapsible::None class="hidden flex-1 md:flex">
                    <SidebarHeader class="gap-3.5 border-b p-4">
                    <div class="flex w-full items-center justify-between">
                        <div class="text-foreground text-base font-medium">
                            // Display title based on current sidebar state
                            { move || match active_sidebar_route.get() {
                                SideBarRoute::Server => "Server",
                                SideBarRoute::Discover => "Discover",
                                SideBarRoute::Servers => "Servers",
                                SideBarRoute::Search => "Search",
                                SideBarRoute::Inbox => "Inbox",
                            }}
                        </div>
                        <Label class="flex items-center gap-2 text-sm">
                            <span>Unreads</span>
                        </Label>
                    </div>
                    <SidebarInput {..} placeholder="Type to search..." />
                    </SidebarHeader>
                    <SidebarContent>
                        <SidebarGroup class="px-0">
                            <SidebarGroupContent>
                                // Conditionally render content based on current sidebar state
                                { move || match active_sidebar_route.get() {
                                    SideBarRoute::Server => view! { <div>"Server Specific Content (e.g., Channels, Members)"</div> }.into_any(),
                                    SideBarRoute::Discover => view! { <div>"Discover Servers Content"</div> }.into_any(),
                                    SideBarRoute::Servers => view! { <div>"Your Servers List Content"</div> }.into_any(),
                                    SideBarRoute::Search => view! { <div>"Global Search Interface"</div> }.into_any(),
                                    SideBarRoute::Inbox => view! { <div>"Inbox Messages/Notifications"</div> }.into_any(),
                                }}
                            </SidebarGroupContent>
                        </SidebarGroup>
                    </SidebarContent>
                </Sidebar>
                <SidebarRail/>
            </Show>
        </Sidebar>
    }
}

#[component]
pub fn ServerMenu(set_active_sidebar_route: WriteSignal<SideBarRoute>) -> impl IntoView {
    let location = use_location();
    let (name, set_name) = signal(String::default());
    let create_server: ServerAction<CreateServer> = ServerAction::new();
    let pending = create_server.pending();
    view! {
    <Dialog>
        <SidebarMenuItem>
            <DropdownMenu>
                <A href="/servers"
                   on:click=move |_| set_active_sidebar_route(get_route_from_path(&location.pathname.get_untracked()))
                >
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
                    <A href="/servers/discover"
                       on:click=move |_| set_active_sidebar_route(get_route_from_path(&location.pathname.get_untracked()))
                    >
                        <DropdownMenuItem close_on_click=true>
                            <IconCompass />
                            "Search" // This "Search" is for Discover servers, not the global search
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
