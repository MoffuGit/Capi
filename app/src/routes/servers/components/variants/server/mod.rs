mod categories;
mod channels;
mod header;

use api::convex::mutations::invitation::CreateInvitation;
use api::server::{CreateCategory, CreateChannel};
use common::convex::{Member, Server};
use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::icons::IconLoaderCircle;
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::dialog::{Dialog, DialogFooter, DialogHeader, DialogPopup, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenuAction,
    SidebarMenuButton, SidebarMenuItem,
};
use crate::routes::servers::components::sidebar::SideBarData;

use self::categories::CategoriesItems;
use self::channels::ChannelsItems;
use self::header::ServerHeader;

#[component]
pub fn ServerSideBar(data: Signal<Option<Vec<SideBarData>>>) -> impl IntoView {
    let create_channel: ServerAction<CreateChannel> = ServerAction::new();
    let create_category: ServerAction<CreateCategory> = ServerAction::new();

    let location = use_location();
    let path = location.pathname;

    let server = Memo::new(move |_| {
        let id = path
            .get()
            .split('/')
            .nth(2)
            .map(|server| server.to_string())
            .unwrap_or_default();
        data.get().and_then(|data| {
            data.iter()
                .find(|SideBarData { server, .. }| server.id == id)
                .map(|data| data.server.clone())
        })
    });

    let member = Memo::new(move |_| {
        let id = path
            .get()
            .split('/')
            .nth(2)
            .map(|server| server.to_string())
            .unwrap_or_default();
        data.get().and_then(|data| {
            data.iter()
                .find(|SideBarData { server, .. }| server.id == id)
                .map(|data| data.member.clone())
        })
    });

    view! {
        <ServerHeader server=server />
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupContent>
                    <PendingChannelItem create_channel=create_channel/>
                    <ChannelsItems server=server/>
                </SidebarGroupContent>
                <PendingCategoryItem create_category=create_category/>
            </SidebarGroup>
            <CategoriesItems server=server />
            <SideBarContextMenu create_channel=create_channel create_category=create_category server=server member=member/>
        </SidebarContent>
    }
}

#[component]
pub fn PendingCategoryItem(create_category: ServerAction<CreateCategory>) -> impl IntoView {
    let input = create_category.input();
    view! {
            {
                move || {
                    input.get().map(|input| {
                        view! {
                            <SidebarGroup>
                                <SidebarGroupLabel>
                                    {input.name.clone()}
                                </SidebarGroupLabel>
                            </SidebarGroup>
                        }
                    })
                }
            }
    }
}

#[component]
pub fn PendingChannelItem(create_channel: ServerAction<CreateChannel>) -> impl IntoView {
    let input = create_channel.input();
    view! {
            {
                move || {
                    input.get().map(|input| {
                        view! {
                            <SidebarMenuItem>
                                <SidebarMenuButton class="min-w-full opacity-50 cursor-not-allowed">
                                    {input.name.clone()}
                                </SidebarMenuButton>
                                <SidebarMenuAction>
                                    <IconLoaderCircle class="animate-spin" />
                                    <span class="sr-only">Loading</span>
                                </SidebarMenuAction>
                            </SidebarMenuItem>
                        }
                    })
                }
            }
    }
}

#[component]
pub fn SideBarContextMenu(
    server: Memo<Option<Server>>,
    member: Memo<Option<Member>>,
    create_channel: ServerAction<CreateChannel>,
    create_category: ServerAction<CreateCategory>,
) -> impl IntoView {
    let create_channel_open = RwSignal::new(false);
    let create_category_open = RwSignal::new(false);
    let invitation_open = RwSignal::new(false);
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="w-full h-full"/>
            <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        create_channel_open.set(true)
                    }
                >
                    "Create Channel"
                </ContextMenuItem>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        create_category_open.set(true)
                    }
                >
                    "Create Category"
                </ContextMenuItem>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        invitation_open.set(true)
                    }
                >
                    "Invitate People"
                </ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
        <InvitationDialog open=invitation_open server=server member=member/>
        <CreateChannelDialog open=create_channel_open server=server create_channel=create_channel/>
        <CreateCategoryDialog open=create_category_open server=server create_category=create_category/>
    }
}

#[component]
pub fn InvitationDialog(
    open: RwSignal<bool>,
    server: Memo<Option<Server>>,
    member: Memo<Option<Member>>,
) -> impl IntoView {
    let invitation: ServerAction<CreateInvitation> = ServerAction::new();

    Effect::new(move |_| {
        if let (Some(server), Some(member)) = (server.get(), member.get()) {
            invitation.dispatch(CreateInvitation {
                server: server.id,
                member: member.id,
            });
        }
    });

    view! {
        <Dialog
            open=open
        >
            <DialogPopup>
                <DialogHeader>
                    <DialogTitle>"Invitation Code"</DialogTitle>
                    // <DialogDescription>
                    //     "Give your new server a personality with a name and an icon. You can always change it later."
                    // </DialogDescription>
                </DialogHeader>
                    <div class="grid gap-2">
                        {
                            move || {
                                invitation.value().get().map(|value| {
                                    value.ok().map(|invitation| view!{
                                        <div>
                                            {invitation}
                                        </div>
                                    })
                                })
                            }
                        }
                    </div>
            </DialogPopup>
        </Dialog>

    }
}

#[component]
pub fn CreateChannelDialog(
    open: RwSignal<bool>,
    create_channel: ServerAction<CreateChannel>,
    server: Memo<Option<Server>>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let pending = create_channel.pending();
    view! {
        <Dialog
            open=open
        >
            <DialogPopup>
                <DialogHeader>
                    <DialogTitle>"Create New Channel"</DialogTitle>
                    // <DialogDescription>
                    //     "Give your new server a personality with a name and an icon. You can always change it later."
                    // </DialogDescription>
                </DialogHeader>
                    <div class="grid gap-2">
                        <Label {..} for="channel-name">Channel Name</Label>
                        <Input
                            {..}
                            id="channel-name"
                            type="text"
                            placeholder="My Awesome Channel"
                            required=true
                            value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>
                <DialogFooter>
                    <button
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(server) = server.get() {
                                    let input = CreateChannel { name: name.get(), server: server.id , category: None };
                                    create_channel.dispatch(input.clone());
                                    open.set(false);
                                }
                            }
                        }
                        disabled=move || pending.get() | server.get().is_none()
                        class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
                    >
                        "Create"
                    </button>
                </DialogFooter>
            </DialogPopup>
        </Dialog>

    }
}

#[component]
pub fn CreateCategoryDialog(
    open: RwSignal<bool>,
    server: Memo<Option<Server>>,
    create_category: ServerAction<CreateCategory>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let pending = create_category.pending();
    view! {
        <Dialog
            open=open
        >
            <DialogPopup>
                <DialogHeader>
                    <DialogTitle>"Create New Category"</DialogTitle>
                    // <DialogDescription>
                    //     "Give your new server a personality with a name and an icon. You can always change it later."
                    // </DialogDescription>
                </DialogHeader>
                    <div class="grid gap-2">
                        <Label {..} for="channel-name">Category Name</Label>
                        <Input
                            {..}
                            id="channel-name"
                            type="text"
                            placeholder="My Awesome Category"
                            required=true
                            value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>
                <DialogFooter>
                    <button
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(server) = server.get() {
                                    let input = CreateCategory { name: name.get(), server: server.id };
                                    create_category.dispatch(input.clone());
                                    open.set(false);
                                }
                            }
                        }
                        disabled=move || pending.get() | server.get().is_none()
                        class="inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
                    >
                        "Create"
                    </button>
                </DialogFooter>
            </DialogPopup>
        </Dialog>

    }
}
