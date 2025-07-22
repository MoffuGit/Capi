use api::convex::mutations::invitation::JoinWithInvitation;
use api::server::CreateServer;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::icons::{
    IconCirclePlus, IconCompass, IconGlobe, IconInbox, IconMessageCircle, IconPencil, IconSearch,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::dialog::{
    Dialog, DialogDescription, DialogFooter, DialogHeader, DialogPopup, DialogTitle,
};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarSeparator,
};
use crate::components::ui::tooltip::{ToolTip, ToolTipContent, ToolTipTrigger};
use crate::routes::servers::components::navbar::Navbar;
use crate::routes::servers::components::servers::ServersItems;
use crate::routes::use_profile;

use super::sidebar::{SideBarData, SideBarOption};

#[component]
pub fn SidebarIcons(
    data: Signal<Option<Vec<SideBarData>>>,
    option: RwSignal<Option<SideBarOption>>,
) -> impl IntoView {
    let set_option = Callback::new(move |_| {
        option.set(None);
    });
    view! {
        <Sidebar
            collapsible=SideBarCollapsible::None
            class="w-[calc(var(--sidebar-width-icon)+1px)]! border-r"
        >
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent class="px-1.5 md:px-0">
                        <SidebarMenu>
                            <Direct set_option=set_option/>
                            <InboxOption option=option/>
                            <SearchOption option=option/>
                            <ServerMenu set_option=set_option/>
                            <SidebarSeparator
                                class="mr-2 data-[orientation=horizontal]:w-4 my-0.5"
                            />
                            <ServersItems data=data set_option=set_option/>
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
pub fn Direct(set_option: Callback<()>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <A href="/servers/me"
                        {..}
                        on:click=move |_| set_option.run(())
                    >
                        <SidebarMenuButton
                            size=crate::components::ui::sidebar::SidebarMenuButtonSize::Sm
                            class="md:h-8 md:p-0 flex items-center justify-center"
                        >
                            <IconMessageCircle class="size-4 text-muted-foreground" />
                        </SidebarMenuButton>
                    </A>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0>
                    "Direct Messages"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

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
                        <IconInbox class="size-4 text-muted-foreground"/>
                    </SidebarMenuButton>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0>
                    "Inbox"
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
                        <IconSearch class="size-4 text-muted-foreground"/>
                    </SidebarMenuButton>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0 >
                    "Search"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

    }
}

#[component]
pub fn ServerMenu(set_option: Callback<()>) -> impl IntoView {
    let create_open = RwSignal::new(false);
    let join_open = RwSignal::new(false);
    view! {
        <SidebarMenuItem>
            <ContextMenu>
                <A
                    href="/servers"
                    {..}
                    on:click=move |_| set_option.run(())
                >
                    <ContextMenuTrigger pointer=false >
                        <ToolTip>
                            <ToolTipTrigger>
                            <SidebarMenuButton
                              class="px-2.5 md:px-2"
                            >
                                <IconGlobe class="size-4 text-muted-foreground"/>
                            </SidebarMenuButton>
                            </ToolTipTrigger>
                            <ToolTipContent >
                                "Servers"
                            </ToolTipContent>
                        </ToolTip>
                    </ContextMenuTrigger>
                </A>
                <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    <ContextMenuItem
                        {..}
                        on:click=move |_| {
                            create_open.set(true);
                        }
                    >
                        <IconPencil/>
                        "Create"
                    </ContextMenuItem>
                    <ContextMenuItem
                        {..}
                        on:click=move |_| {
                            join_open.set(true);
                        }
                    >
                        <IconCirclePlus/>
                        "Join"
                    </ContextMenuItem>
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
        <CreateServerDialog open=create_open/>
        <JoinServerDialog open=join_open/>
    }
}

#[component]
pub fn JoinServerDialog(open: RwSignal<bool>) -> impl IntoView {
    let user = use_profile();
    let join_server: ServerAction<JoinWithInvitation> = ServerAction::new();
    let (name, set_name) = signal(String::default());
    let pending = join_server.pending();
    view! {
        <Dialog open=open>
            <DialogPopup>
                <DialogHeader>
                    <DialogTitle >"Join Server"</DialogTitle>
                    <DialogDescription>
                        "use your invitation code"
                    </DialogDescription>
                </DialogHeader>
                    <div class="grid gap-2">
                        <Label {..} for="invitation">Invitation Code</Label>
                        <Input
                            {..}
                            id="invitation"
                            type="text"
                            placeholder="Your code"
                            required=true
                            value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>
                <DialogFooter>
                    <div/>
                    <button
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(user) = user.get() {
                                    join_server.dispatch(JoinWithInvitation {
                                        invitation: name.get(),
                                        user: user.id
                                    });
                                }
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

// pub struct CreateServer {
//     name: String,
// }

#[component]
pub fn CreateServerDialog(open: RwSignal<bool>) -> impl IntoView {
    let create_server: ServerAction<CreateServer> = ServerAction::new();
    let (name, set_name) = signal(String::default());
    let pending = create_server.pending();
    view! {
        <Dialog open=open>
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
