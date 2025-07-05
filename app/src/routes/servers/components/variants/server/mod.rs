mod categories;
mod channels;

use api::server::{CreateCategory, CreateChannel};
use common::convex::Server;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::icons::{IconLoaderCircle, IconPlus, IconTrash};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::context::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuTrigger,
};
use crate::components::ui::dialog::{Dialog, DialogFooter, DialogHeader, DialogPopup, DialogTitle};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::sidebar::{
    SidebarContent, SidebarGroup, SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel,
    SidebarHeader, SidebarMenu, SidebarMenuAction, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem,
};

use self::categories::CategoriesItems;
use self::channels::ChannelsItems;

#[component]
pub fn ServerSideBar(servers: RwSignal<Option<Vec<Server>>>) -> impl IntoView {
    let create_channel: ServerAction<CreateChannel> = ServerAction::new();
    let create_category: ServerAction<CreateCategory> = ServerAction::new();

    let params = use_params_map();
    let server = Signal::derive(move || {
        let id = params.get().get("server");
        let servers = servers.get();
        id.and_then(|id| {
            servers.map(|servers| servers.iter().find(|server| server.id == id).cloned())
        })
        .flatten()
    });

    // Signals to control the open state of each dialog
    let show_create_channel_dialog = RwSignal::new(false);
    let show_create_category_dialog = RwSignal::new(false);

    let channel_pending = create_channel.pending();
    let channel_value = create_channel.input();
    let channel_result = create_channel.value();

    let category_pending = create_category.pending();
    let category_value = create_category.input();
    let category_result = create_category.value();

    let last_channel_input: RwSignal<Option<CreateChannel>> = RwSignal::new(None);
    let last_category_input: RwSignal<Option<CreateCategory>> = RwSignal::new(None);

    view! {
            {
                move || {
                    server.get().map(|server| {
                        let name = StoredValue::new(server.name.clone());
                        let image_url = StoredValue::new(server.image_url.clone());
                        view!{
                            <SidebarHeader class="flex w-full">
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton size=SidebarMenuButtonSize::Lg>
                                            <Avatar class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                                <AvatarImage url=image_url.get_value()/>
                                                <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                    {name.get_value().chars().next()}
                                                </AvatarFallback>
                                            </Avatar>
                                            <div class="grid flex-1 text-left text-base leading-tight">
                                                <span class="truncate font-medium">
                                                    {
                                                        name.get_value()
                                                    }
                                                </span>
                                            </div>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarHeader>
                        }
                    })
                }
            }
            <ContextMenu>
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <Show when=move || channel_pending.get()>
                                {
                                    move || {
                                        channel_value.get().map(|input| {
                                            view! {
                                                <SidebarMenuItem>
                                                    <SidebarMenuButton class="min-w-full opacity-50 cursor-not-allowed">
                                                        {input.name.clone()}
                                                    </SidebarMenuButton>
                                                    {
                                                        move || {
                                                            match channel_result.get() {
                                                                None => view!{
                                                                    <SidebarMenuAction>
                                                                        <IconLoaderCircle class="animate-spin" />
                                                                        <span class="sr-only">Loading</span>
                                                                    </SidebarMenuAction>
                                                                }.into_any(),
                                                                Some(Err(_)) => view!{
                                                                        <SidebarMenuAction class="bg-destructive text-white hover:bg-destructive/90">
                                                                            <IconTrash  />
                                                                            <span class="sr-only">Error</span>
                                                                        </SidebarMenuAction>
                                                                }.into_any(),
                                                                _ => ().into_any()
                                                            }
                                                        }
                                                    }
                                                </SidebarMenuItem>
                                            }
                                        })
                                    }
                                }
                            </Show>
                            <ChannelsItems server=server/>
                        </SidebarGroupContent>
                    </SidebarGroup>
                        <Show when=move || category_pending.get()>
                            {
                                move || {
                                    category_value.get().map(|category| {
                                        view! {
                                            <SidebarGroup>
                                                <SidebarGroupLabel>
                                                    {category.name}
                                                </SidebarGroupLabel>
                                                {
                                                    move || {
                                                        match channel_result.get() {
                                                            None => view!{
                                                                <SidebarGroupAction>
                                                                    <IconLoaderCircle class="animate-spin" />
                                                                    <span class="sr-only">Loading</span>
                                                                </SidebarGroupAction>
                                                            }.into_any(),
                                                            Some(Err(_)) => view!{
                                                                    <SidebarGroupAction class="bg-destructive text-white hover:bg-destructive/90">
                                                                        <IconTrash  />
                                                                        <span class="sr-only">Error</span>
                                                                    </SidebarGroupAction>
                                                            }.into_any(),
                                                            _ => ().into_any()
                                                        }
                                                    }
                                                }
                                            </SidebarGroup>
                                        }
                                    })
                                }
                            }
                        </Show>
                    <CategoriesItems server=server/>
                    <ContextMenuTrigger class="w-full h-full"/>
                </SidebarContent>
                <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    // Use on:select to set the signal, allowing ContextMenu to close naturally
                    <ContextMenuItem {..} on:click=move |_| show_create_channel_dialog.set(true)>
                        "Create Channel"
                    </ContextMenuItem>
                    <ContextMenuItem {..} on:click=move |_| show_create_category_dialog.set(true)>
                        "Create Category"
                    </ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>

            {
                move || {
                    server.get().map(|server| {
                        view! {
                            <CreateChannelDialog
                                create_channel=create_channel
                                show_create_channel_dialog=show_create_channel_dialog
                                server=server.clone()
                                last_channel_input=last_channel_input
                            />
                            <CreateCategoryDialog
                                create_category=create_category
                                show_create_category_dialog=show_create_category_dialog
                                server=server
                                last_category_input=last_category_input
                            />
                        }
                })
                }
            }
    }
}

#[component]
pub fn CreateChannelDialog(
    show_create_channel_dialog: RwSignal<bool>,
    create_channel: ServerAction<CreateChannel>,
    server: Server,
    last_channel_input: RwSignal<Option<CreateChannel>>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let pending = create_channel.pending();
    let id = StoredValue::new(server.id);
    view! {
        <Dialog
            open=show_create_channel_dialog
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
                                let input = CreateChannel { name: name.get(), server: id.get_value() , category: None };
                                create_channel.dispatch(input.clone());
                                last_channel_input.set(Some(input));
                                show_create_channel_dialog.set(false);
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

#[component]
pub fn CreateCategoryDialog(
    show_create_category_dialog: RwSignal<bool>,
    server: Server,
    create_category: ServerAction<CreateCategory>,
    last_category_input: RwSignal<Option<CreateCategory>>,
) -> impl IntoView {
    let (name, set_name) = signal(String::default());
    let pending = create_category.pending();
    let id = StoredValue::new(server.id);
    view! {
        <Dialog
            open=show_create_category_dialog
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
                                let input = CreateCategory { name: name.get(), server: id.get_value() };
                                create_category.dispatch(input.clone());
                                last_category_input.set(Some(input));
                                show_create_category_dialog.set(false);
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
