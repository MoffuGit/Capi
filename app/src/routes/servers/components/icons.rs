use api::server::CreateServer;
use leptos::prelude::*;
use leptos_router::components::A;

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
    SidebarSeparator,
};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::routes::servers::components::navbar::Navbar;
use crate::routes::servers::components::servers::ServersItems;

use super::sidebar::{SideBarData, SideBarOption};

#[component]
pub fn SidebarIcons(
    data: RwSignal<Option<Vec<SideBarData>>>,
    option: RwSignal<Option<SideBarOption>>,
) -> impl IntoView {
    view! {
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
                            <InboxOption option=option/>
                            <SearchOption option=option/>
                            <ServerMenu />
                            <SidebarSeparator
                                class="mr-2 data-[orientation=horizontal]:w-4 my-0.5"
                            />
                            <ServersItems data=data />
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter>
                <Navbar/>
            </SidebarFooter>
        </Sidebar>
    }
}

#[component]
pub fn InboxOption(option: RwSignal<Option<SideBarOption>>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <SidebarMenuButton
                        class="px-2.5 md:px-2"
                        {..}
                        on:click=move |_| {
                            option.set(Some(SideBarOption::Inbox))
                        }
                    >
                        <IconInbox/>
                    </SidebarMenuButton>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0 arrow=true>
                    "Search"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

    }
}

#[component]
pub fn SearchOption(option: RwSignal<Option<SideBarOption>>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <SidebarMenuButton
                        class="px-2.5 md:px-2"
                        {..}
                        on:click=move |_| {
                            option.set(Some(SideBarOption::Search))
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
            <ContextMenu>
                <A
                    href="/servers"
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
