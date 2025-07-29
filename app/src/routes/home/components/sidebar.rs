use api::presence::GetUserStatus;
use api::server::GetServers;
pub use api::server::SideBarData;
use api::sidebar::SideBarState;
use convex_client::leptos::UseQuery;
use leptos::prelude::*;
use leptos_dom::warn;

use crate::components::auth::use_auth;
use crate::components::icons::{IconHeadphones, IconMic, IconSettings};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::dropwdown::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuTrigger,
};
use crate::components::ui::sidebar::{
    SideBarCollapsible as SideBarCollapsibleType, Sidebar, SidebarRail,
};
use crate::routes::home::components::collapsible::SidebarCollapsible;
use crate::routes::home::components::icons::SidebarIcons;
use crate::routes::home::components::user_settings::DialogUserSettings;
use crate::routes::{use_profile, SideBarRoute};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarOption {
    Search,
    Inbox,
}

#[component]
pub fn SideBar(
    route: Memo<SideBarRoute>,
    option: RwSignal<Option<SideBarOption>>,
) -> impl IntoView {
    let auth = use_auth().auth();

    let data = UseQuery::new(move || {
        auth.get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|auth| GetServers { auth: auth.id })
    });

    Effect::new(move |_| {
        warn!("{:?}", data.get());
    });

    let data = Signal::derive(move || data.get().and_then(|res| res.ok()));

    let state = MaybeProp::derive(move || {
        if option.get().is_none()
            && (route.get() == SideBarRoute::Servers || route.get() == SideBarRoute::Discover)
        {
            Some(SideBarState::Collapsed)
        } else {
            None
        }
    });

    view! {
        <Sidebar state=state collapsible=SideBarCollapsibleType::Icon class="overflow-hidden *:data-[sidebar=sidebar]:flex-row">
            <SidebarIcons data=data option=option/>
            <SidebarCollapsible data=data route=route option=option/>
            <Profile/>
            <SidebarRail/>
        </Sidebar>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_profile();

    let status = UseQuery::new(move || user.get().map(|user| GetUserStatus { user: user.id }));

    let open_user_settings = RwSignal::new(false);
    view! {
        <div class="bg-background h-8 shadow-md border rounded-lg flex items-center bottom-2 left-2 absolute group-data-[state=collapsed]:w-8 group-data-[state=expanded]:p-1 group-data-[state=expanded]:w-[calc(var(--sidebar-width)-18px)] group-data-[state=expanded]:h-13 transition-all ease-in-out-cubic duration-200 overflow-hidden">
            <DropdownMenu>
                <DropdownMenuTrigger class="h-full flex items-center hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 rounded-lg group-data-[state=expanded]:p-1 min-w-0">
                    <Avatar class="flex relative bg-accent aspect-square size-8 items-center justify-center rounded-lg overflow-visible">
                        {
                            move || {
                                user.get().map(|user| {
                                    let name = StoredValue::new(user.name);
                                    view!{
                                                    <AvatarImage url=user.image_url class="rounded-lg"/>
                                                    <AvatarFallback class="rounded-lg select-none bg-transparent">
                                                        {name.get_value().chars().next()}
                                                    </AvatarFallback>
                                    }
                                })
                            }
                        }
                    </Avatar>
                    <div class="flex justify-between flex-col h-full px-1 group-data-[state=collapsed]:opacity-0 group-data-[state=expqnded]:opacity-100 min-w-0">
                        <div class="text-sm truncate font-light">
                            {move || user.get().map(|user| user.name)}
                        </div>
                        <Badge class="h-4 rounded-sm px-1.5" variant=BadgeVariant::Outline>
                            {move || status.get().and_then(|res| res.ok()).flatten().map(|status| {
                                status.to_string()
                            })}
                        </Badge>
                    </div>
                </DropdownMenuTrigger>
                <DropdownMenuContent side=MenuSide::Top align=MenuAlign::Start side_of_set=-10.0>
                    <DropdownMenuGroup>
                        <DropdownMenuLabel>
                            {move || {
                                user.get().map(|user| user.name)
                            }}
                        </DropdownMenuLabel>
                        <DropdownMenuItem
                            on:click=move |_| {
                                open_user_settings.set(true);
                            }
                        >
                            <IconSettings/>
                            "Settings"
                        </DropdownMenuItem>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
            <div class="flex justify-center w-auto group-data-[state=expanded]:shrink-0 group-data-[state=collapsed]:w-0 items-center ml-auto gap-2 overflow-hidden px-1 group-data-[state=collapsed]:opacity-0 group-data-[state=expqnded]:opacity-100 transition-opacity ease-out">
                <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                    <IconMic/>
                </Button>
                <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost>
                    <IconHeadphones/>
                </Button>
            </div>
        </div>
        <DialogUserSettings open=open_user_settings />
    }
}
