use api::presence::GetUserStatus;
use common::convex::PresenceStatus;
use convex_client::leptos::{Mutation, UseMutation, UseQuery};
use leptos::prelude::*;
use serde::Serialize;
use strum::IntoEnumIterator;

use crate::components::auth::use_auth;
use crate::components::icons::{IconHeadphones, IconMic, IconSettings};
use crate::components::ui::avatar::*;
use crate::components::ui::badge::*;
use crate::components::ui::button::*;
use crate::components::ui::dropwdown::*;
use crate::routes::home::components::user_settings::DialogUserSettings;
use crate::routes::use_profile;
use capi_primitives::menu::{MenuAlign, MenuSide};

#[derive(Debug, Clone, Serialize)]
pub struct SetUserStatus {
    auth: i64,
    status: PresenceStatus,
}

impl Mutation for SetUserStatus {
    type Output = ();

    fn name(&self) -> String {
        "presence:patchUserStatus".into()
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_profile();

    let status = UseQuery::new(move || user.get().map(|user| GetUserStatus { user: user.id }));
    let auth = use_auth().auth();

    let set_status = UseMutation::new::<SetUserStatus>();

    let open_user_settings = RwSignal::new(false);

    view! {
        <div class="bg-background h-8 shadow-md border rounded-lg flex items-center bottom-2 left-2 absolute group-data-[state=collapsed]:w-8 group-data-[state=expanded]:p-1 group-data-[state=expanded]:w-[calc(var(--sidebar-width)-18px)] group-data-[state=expanded]:h-13 transition-all ease-in-out-cubic duration-200 overflow-hidden">
            <DropdownMenu>
                <DropdownMenuTrigger class="h-full active:scale-[.98] duration-150 transition-[scale] flex items-center hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50 rounded-lg group-data-[state=expanded]:p-1 min-w-0">
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
                    <div class="flex justify-between flex-col h-[32px] w-[152px] pl-2 pr-1 group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 transition-all duration-200 ease-out group-data-[state=collapsed]:-translate-x-3.5">
                        <div class="text-xs truncate font-light">
                            {move || user.get().map(|user| user.name)}
                        </div>
                            {move || status.get().and_then(|res| res.ok()).flatten().map(|status| {
                                view!{
                                    <Badge class="h-4 rounded-sm px-1.5" variant=Signal::derive(move || {
                                        match status {
                                            PresenceStatus::Online => BadgeVariant::Online,
                                            PresenceStatus::NotDisturb => BadgeVariant::NotDisturb,
                                            PresenceStatus::Idle => BadgeVariant::Idle,
                                            _ => BadgeVariant::Secondary
                                        }
                                    })>
                                        {
                                            status.to_string()
                                        }
                                    </Badge>
                                }
                            })}
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
                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>
                                "Status"
                            </DropdownMenuSubTrigger>
                            <DropdownMenuSubContent side=MenuSide::Right align=MenuAlign::Center>
                                {
                                    move || {
                                        status.get().and_then(|res| res.ok()).flatten().map(|status| {
                                            let status = RwSignal::new(status.to_string());
                                            view!{
                                                <DropdownMenuRadioGroup value=status>
                                                    {
                                                        PresenceStatus::iter().map(|status| {
                                                            view!{
                                                                <DropdownMenuRadioItem value=status.to_string() on:click=move |_| {
                                                                    if let Some(auth)= auth.get().and_then(|res| res.ok()).flatten() {
                                                                        set_status.dispatch(SetUserStatus { auth: auth.id, status });
                                                                    }
                                                                }>
                                                                    {status.to_string()}
                                                                </DropdownMenuRadioItem>
                                                            }
                                                        }).collect_view()
                                                    }
                                                </DropdownMenuRadioGroup>
                                            }
                                        })
                                    }
                                }
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                    </DropdownMenuGroup>
                </DropdownMenuContent>
            </DropdownMenu>
            <div class="flex justify-center w-auto group-data-[state=expanded]:shrink-0 group-data-[state=collapsed]:w-0 items-center ml-auto gap-2 overflow-hidden px-1 group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 transition-opacity ease-out duration-250">
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
