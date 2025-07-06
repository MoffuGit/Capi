use api::convex::Query;
use api::server::CreateServer;
use common::convex::{Member, Server};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::MouseEvent;

use super::navbar::Navbar;
use super::variants::ServerSideBar;
use crate::components::auth::use_auth;
use crate::components::icons::{
    IconCirclePlus, IconCommand, IconCompass, IconGlobe, IconInbox, IconSearch,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::dialog::{
    Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPopup, DialogTitle, DialogTrigger,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarHeader, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize, SidebarMenuItem,
    SidebarRail, SidebarSeparator,
};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::hooks::sycn::SyncSignal;
use crate::routes::servers::components::servers::ServersItems;
use crate::routes::servers::components::variants::{
    DiscoverSideBar, InboxSideBar, PrivateSideBar, SearchSideBar, ServersSideBar,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarRoute {
    Server,
    Discover,
    Servers,
    Private,
}

/// Helper function to derive SideBarRoute from a given path
fn get_route_from_path(path: &str) -> SideBarRoute {
    match path.split('/').nth(2) {
        None | Some("") => SideBarRoute::Servers, // Covers /servers and /servers/
        Some("discover") => SideBarRoute::Discover,
        Some("me") => SideBarRoute::Private,
        _ => SideBarRoute::Server, // Covers /servers/{id} or anything else
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SideBarData {
    pub server: Server,
    pub member: Member,
}

#[component]
pub fn SideBar() -> impl IntoView {
    let location = use_location();
    let side_bar_route = Memo::new(move |_| {
        let path = location.pathname.get();
        get_route_from_path(&path)
    });

    let (inbox, set_inbox) = signal(false);
    let (search, set_search) = signal(false);

    let on_click = move |_| {
        set_inbox(false);
        set_search(false);
    };

    let auth = use_auth();

    let data: SyncSignal<Vec<SideBarData>> = SyncSignal::new(Memo::new(move |_| {
        auth.user.get().flatten().map(|user| Query {
            name: "user:getServers".to_string(),
            args: json!({
                "user": user.id
            }),
        })
    }));

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
                            {..}
                            on:click=on_click
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
                                        {..}
                                        on:click=move |_| {
                                            set_inbox(false);
                                            set_search(true);
                                        }
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
                                        {..}
                                        on:click=move |_| {
                                            set_search(false);
                                            set_inbox(true);
                                        }
                                      >
                                        <IconInbox/>
                                      </SidebarMenuButton>
                                    </ToolTipTrigger>
                                    <ToolTipContent side_of_set=3.0 arrow=true>
                                        "Inbox"
                                    </ToolTipContent>
                                </ToolTip>
                                    </SidebarMenuItem>
                            <ServerMenu on_click=Callback::new(on_click) />
                            <SidebarSeparator
                                class="mr-2 data-[orientation=horizontal]:w-4 my-0.5"
                            />
                            <ServersItems data=data.signal on_click=Callback::new(on_click)/>
                        </SidebarMenu>
                    </SidebarGroupContent>
                  </SidebarGroup>
                </SidebarContent>
                <SidebarFooter>
                    <Navbar/>
                </SidebarFooter>
            </Sidebar>

            <Sidebar collapsible=SideBarCollapsible::None class="flex-1 md:flex min-w-[250px]">
                {
                    move || match side_bar_route.get() {
                        // (true, _, _) => view!{ <InboxSideBar/> }.into_any(),
                        // (_, true, _) => view!{ <SearchSideBar/> }.into_any(),
                        SideBarRoute::Server  => view!{<ServerSideBar data=data.signal/>}.into_any(),
                        SideBarRoute::Discover  => view!{<DiscoverSideBar/>}.into_any(),
                        SideBarRoute::Servers  => view!{<ServersSideBar/>}.into_any(),
                        SideBarRoute::Private  => view!{<PrivateSideBar/>}.into_any(),
                    }

                }
            </Sidebar>
            <SidebarRail/>
        </Sidebar>
    }
}

#[component]
pub fn ServerMenu(on_click: Callback<MouseEvent>) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let create_server: ServerAction<CreateServer> = ServerAction::new();
    let pending = create_server.pending();
    view! {
    <Dialog>
        <SidebarMenuItem>
            <ContextMenu>
                <A
                    href="/servers"
                    {..}
                    on:click=move |evt| on_click.run(evt)
                >
                    <ContextMenuTrigger pointer=false >
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
                    </ContextMenuTrigger>
                </A>
                <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    <DialogTrigger as_child=true>
                        <ContextMenuItem>
                            <IconCirclePlus/>
                            "Create"
                        </ContextMenuItem>
                    </DialogTrigger>
                    <A
                        href="/servers/discover"
                        {..}
                        on:click=move |evt| on_click.run(evt)
                    >
                        <ContextMenuItem close_on_click=true>
                            <IconCompass />
                            "Search" // This "Search" is for Discover servers, not the global search
                        </ContextMenuItem>
                    </A>
                </ContextMenuContent>
            </ContextMenu>
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
