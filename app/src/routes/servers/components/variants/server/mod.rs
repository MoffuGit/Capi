mod categories;
mod channels;
mod header;

use api::category::GetCategories;
use common::convex::{Category, Member, Server};
use convex_client::leptos::{Mutation, UseMutation, UseQuery};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::Serialize;

use crate::components::auth::use_auth;
use crate::components::icons::{
    IconBox, IconChevronDown, IconLink, IconLoaderCircle, IconPlus, IconSettings,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::roles::{
    CanCreateInvitation, CanManageCategories, CanManageChannels, CanManageServerSettings,
    RolesProvider,
};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuLabel, ContextMenuTrigger,
};
use crate::components::ui::dialog::{Dialog, DialogFooter, DialogHeader, DialogPopup, DialogTitle};
use crate::components::ui::dropwdown::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuTrigger,
};
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

#[derive(Debug, Serialize, Clone)]
pub struct CreateChannel {
    name: String,
    server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    auth: i64,
}

impl Mutation for CreateChannel {
    type Output = ();

    fn name(&self) -> String {
        "channel:create".into()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateCategory {
    name: String,
    server: String,
    auth: i64,
}

impl Mutation for CreateCategory {
    type Output = ();

    fn name(&self) -> String {
        "category:create".into()
    }
}

#[component]
pub fn ServerSideBar(data: Signal<Option<Vec<SideBarData>>>) -> impl IntoView {
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

    let roles = Memo::new(move |_| {
        let id = path
            .get()
            .split('/')
            .nth(2)
            .map(|server| server.to_string())
            .unwrap_or_default();
        data.get().and_then(|data| {
            data.iter()
                .find(|SideBarData { server, .. }| server.id == id)
                .map(|data| data.roles.clone())
        })
    });

    let categories = UseQuery::new(move || {
        server
            .get()
            .map(|server| GetCategories { server: server.id })
    });

    let categories = Signal::derive(move || categories.get().and_then(|res| res.ok()));

    view! {
        <RolesProvider roles=Signal::derive(move || roles.get().unwrap_or_default())>
            <ServerHeader server=server />
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <ChannelsItems server=server/>
                    </SidebarGroupContent>
                </SidebarGroup>
                <CategoriesItems server=server categories=categories />
                <SideBarContextMenu categories=categories server=server member=member/>
            </SidebarContent>
        </RolesProvider>
    }
}

#[component]
pub fn PendingCategoryItem(
    create_category: Action<CreateCategory, Result<(), String>>,
) -> impl IntoView {
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
pub fn PendingChannelItem(
    create_channel: Action<CreateChannel, Result<(), String>>,
) -> impl IntoView {
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
pub fn ServerContextMenuData(
    #[prop(into)] server: Signal<Option<Server>>,
    #[prop(into)] member: Signal<Option<Member>>,
    categories: Signal<Option<Vec<Category>>>,
    #[prop(optional)] category: Option<Category>,
) -> impl IntoView {
    let create_channel = UseMutation::new::<CreateChannel>();
    let create_category = UseMutation::new::<CreateCategory>();
    let create_channel_open = RwSignal::new(false);
    let create_category_open = RwSignal::new(false);
    let invitation_open = RwSignal::new(false);
    view! {
        <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                {move || {
                    server.get().map(|server| view!{
                        <ContextMenuLabel class="capitalize">
                            {server.name}
                        </ContextMenuLabel>
                    })
                }}
            <CanManageChannels>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        create_channel_open.set(true)
                    }
                >
                    <IconPlus/>
                    "Create Channel"
                </ContextMenuItem>
            </CanManageChannels>
            <CanManageCategories>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        create_category_open.set(true)
                    }
                >
                    <IconBox/>
                    "Create Category"
                </ContextMenuItem>
            </CanManageCategories>
            <CanCreateInvitation>
                <ContextMenuItem {..}
                    on:click=move |_| {
                        invitation_open.set(true)
                    }
                >
                    <IconLink />
                    "Invitate People"
                </ContextMenuItem>
            </CanCreateInvitation>
            <CanManageServerSettings>
                <ContextMenuItem >
                    <IconSettings />
                    "Settings"
                </ContextMenuItem>
            </CanManageServerSettings>
        </ContextMenuContent>
        <InvitationDialog open=invitation_open server=server member=member/>
        <CreateChannelDialog category=category categories=categories open=create_channel_open server=server create_channel=create_channel/>
        <CreateCategoryDialog open=create_category_open server=server create_category=create_category/>
    }
}

#[component]
pub fn SideBarContextMenu(
    #[prop(into)] server: Signal<Option<Server>>,
    #[prop(into)] member: Signal<Option<Member>>,
    categories: Signal<Option<Vec<Category>>>,
) -> impl IntoView {
    view! {
        <ContextMenu>
            <ContextMenuTrigger class="w-full h-full"/>
            <ServerContextMenuData
                categories=categories
                server=server
                member=member
            />
        </ContextMenu>
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateInvitation {
    server: String,
    member: String,
    #[serde(rename = "expiresInMinutes")]
    expires: f64,
}

impl Mutation for CreateInvitation {
    type Output = String;

    fn name(&self) -> String {
        "invitations:createInvitation".into()
    }
}

#[component]
pub fn InvitationDialog(
    open: RwSignal<bool>,
    server: Signal<Option<Server>>,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let invitation = UseMutation::new();

    Effect::new(move |_| {
        if let (Some(server), Some(member)) = (server.get(), member.get()) {
            invitation.dispatch(CreateInvitation {
                server: server.id,
                member: member.id,
                expires: 1.0 * 60.0 * 7.0,
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
    create_channel: Action<CreateChannel, Result<(), String>>,
    server: Signal<Option<Server>>,
    categories: Signal<Option<Vec<Category>>>,
    category: Option<Category>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let selected_category = RwSignal::new(category);
    let pending = create_channel.pending();
    let auth = use_auth().auth();
    view! {
        <Dialog
            open=open
        >
            <DialogPopup>
                <DialogHeader>
                    <div class="text-sm flex h-8 items-center px-2">
                        <span class="text-foreground/70">
                            "Add channel to"
                        </span>
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <Button variant=ButtonVariants::Ghost size=ButtonSizes::Sm class="gap-1 mx-1 !p-1">
                                    {
                                        move || {
                                            server.get().map(|server| {
                                                view!{
                                                    <Avatar class="flex bg-accent aspect-square size-5 items-center justify-center rounded-lg group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                                                        <AvatarImage url=server.image_url/>
                                                        <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                            {server.name.chars().next()}
                                                        </AvatarFallback>
                                                    </Avatar>

                                                }
                                            })
                                        }
                                    }
                                    <Show when=move || selected_category.get().is_some()>
                                        {
                                            move || {
                                                selected_category.get().map(|category| {
                                                    view!{
                                                        <span class="capitalize font-medium">
                                                            {category.name}
                                                        </span>
                                                    }
                                                })
                                            }
                                        }
                                    </Show>
                                    <Show when=move || selected_category.get().is_none()>
                                        <span class="capitalize font-medium">
                                            {move || {
                                                server.get().map(|server| {
                                                    server.name
                                                })
                                            }}
                                        </span>
                                    </Show>
                                    <IconChevronDown />
                                </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent side=MenuSide::Bottom align=MenuAlign::Center>
                                <DropdownMenuGroup>
                                    <DropdownMenuLabel>
                                        "Categories"
                                    </DropdownMenuLabel>
                                    <For
                                        each=move || categories.get().unwrap_or_default()
                                        key=|category| category.id.clone()
                                        children=move |category| {
                                            let name = StoredValue::new(category.name.clone());
                                            view!{
                                                <DropdownMenuItem
                                                    close_on_click=true
                                                    on:click=move |_| {
                                                        selected_category.set(Some(category.clone()));
                                                    }
                                                >
                                                    {name.get_value()}
                                                </DropdownMenuItem>
                                            }
                                        }
                                    />
                                </DropdownMenuGroup>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                </DialogHeader>
                    <div class="grid gap-2">
                        <Label class="px-2" {..} for="channel-name">Channel Name</Label>
                        <Input
                            {..}
                            id="channel-name"
                            type="text"
                            placeholder="New Channel"
                            required=true
                            value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>
                <DialogFooter>
                    <Transition>
                        <Button
                            variant=ButtonVariants::Secondary
                            size=ButtonSizes::Sm
                            on:click=move |_| {
                                if !name.get().is_empty() {
                                    if let Some(server) = server.get() {
                                        if let Some(user) = auth.get().and_then(|res|res.ok()).flatten() {
                                            let input = CreateChannel { name: name.get(), server: server.id , category: selected_category.get().map(|category| category.id), auth: user.id };
                                            create_channel.dispatch(input);
                                            open.set(false);
                                        }
                                    }
                                }
                            }
                            disabled=Signal::derive(move || pending.get() | server.get().is_none())
                        >
                            "Create"
                        </Button>
                    </Transition>
                </DialogFooter>
            </DialogPopup>
        </Dialog>

    }
}

#[component]
pub fn CreateCategoryDialog(
    open: RwSignal<bool>,
    server: Signal<Option<Server>>,
    create_category: Action<CreateCategory, Result<(), String>>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let pending = create_category.pending();
    let auth = use_auth().auth();
    view! {
        <Dialog
            open=open
        >
            <DialogPopup>
                <DialogHeader>
                    <div class="text-sm flex h-8 items-center px-2">
                        <span class="text-foreground/70">
                            "Add category to"
                        </span>
                            {
                                move || {
                                    server.get().map(|server| {
                                        view!{
                                            <Avatar class="flex bg-accent aspect-square size-5 mx-1 items-center justify-center rounded-lg group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                                                <AvatarImage url=server.image_url/>
                                                <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                    {server.name.chars().next()}
                                                </AvatarFallback>
                                            </Avatar>

                                        }
                                    })
                                }
                            }
                        <span class="capitalize font-medium">
                            {move || {
                                server.get().map(|server| {
                                    server.name
                                })
                            }}
                        </span>
                    </div>
                </DialogHeader>
                <div class="grid gap-2">
                    <Label class="px-2" {..} for="channel-name">Category Name</Label>
                    <Input
                        {..}
                        id="channel-name"
                        type="text"
                        placeholder="New Category"
                        required=true
                        value=name
                        on:input=move |ev| set_name(event_target_value(&ev))
                    />
                </div>
                <DialogFooter>
                    <Button
                        variant=ButtonVariants::Secondary
                        size=ButtonSizes::Sm
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(server) = server.get() {
                                    if let Some(user) = auth.get().and_then(|res|res.ok()).flatten() {
                                        let input = CreateCategory { name: name.get(), server: server.id, auth: user.id };
                                        create_category.dispatch(input.clone());
                                    }
                                    open.set(false);
                                }
                            }
                        }
                        disabled=Signal::derive(move || pending.get() | server.get().is_none())
                    >
                        "Create"
                    </Button>
                </DialogFooter>
            </DialogPopup>
        </Dialog>

    }
}
