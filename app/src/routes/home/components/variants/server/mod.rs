mod categories;
mod channels;
mod header;

use api::category::GetCategories;
use api::channel::GetChannels;
use api::server::ServerData;
use common::convex::{Category, Member, Server};
use convex_client::leptos::{Mutation, UseMutation, UseQuery};
use leptos::prelude::*;
use leptos_dom::log;
use leptos_router::hooks::use_location;
use serde::Serialize;

use crate::components::icons::{IconBox, IconLink, IconPlus, IconSettings};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::roles::*;
use crate::components::ui::context::*;
use crate::components::ui::dialog::*;
use crate::components::ui::sidebar::*;
use crate::routes::home::components::dialogs::create_category::CreateCategoryDialog;
use crate::routes::home::components::dialogs::create_channel::CreateChannelDialog;
use crate::routes::home::components::server_settings::DialogServerSettings;

use self::categories::CategoriesItems;
use self::channels::ChannelsItems;
use self::header::ServerHeader;

#[component]
pub fn ServerSideBar(data: Signal<Option<Vec<ServerData>>>) -> impl IntoView {
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
                .find(|ServerData { server, .. }| server.id == id)
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
                .find(|ServerData { server, .. }| server.id == id)
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
                .find(|ServerData { server, .. }| server.id == id)
                .map(|data| data.roles.clone())
        })
    });

    let categories = UseQuery::new(move || {
        server
            .get()
            .map(|server| GetCategories { server: server.id })
    });

    let categories = Signal::derive(move || categories.get().and_then(|res| res.ok()));

    let channels = UseQuery::new(move || {
        server.get().map(|server| GetChannels {
            server: server.id,
            category: None,
        })
    });

    view! {
        <RolesProvider roles=Signal::derive(move || roles.get().unwrap_or_default())>
            <ServerHeader server=server />
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent>
                        <ChannelsItems channels=channels/>
                    </SidebarGroupContent>
                </SidebarGroup>
                <CategoriesItems server=server categories=categories />
                <SideBarContextMenu categories=categories server=server member=member/>
            </SidebarContent>
        </RolesProvider>
    }
}

#[component]
pub fn ServerContextMenuData(
    #[prop(into)] server: Signal<Option<Server>>,
    #[prop(into)] member: Signal<Option<Member>>,
    categories: Signal<Option<Vec<Category>>>,
    #[prop(optional)] category: Option<Category>,
) -> impl IntoView {
    let create_channel_open = RwSignal::new(false);
    let create_category_open = RwSignal::new(false);
    let invitation_open = RwSignal::new(false);
    let settings_open = RwSignal::new(false);
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
                <ContextMenuItem {..}
                    on:click=move |_| {
                        log!("you click it");
                        settings_open.set(true)
                    }
                >
                    <IconSettings />
                    "Settings"
                </ContextMenuItem>
            </CanManageServerSettings>
        </ContextMenuContent>
        <DialogServerSettings open=settings_open server=server/>
        <InvitationDialog open=invitation_open server=server member=member/>
        <CreateChannelDialog category=category categories=categories open=create_channel_open server=server/>
        <CreateCategoryDialog open=create_category_open server=server/>
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
