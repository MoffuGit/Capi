use common::convex::{Category, Server};
use convex_client::leptos::Mutation;
use convex_client::leptos::UseMutation;
use leptos::prelude::*;
use serde::Serialize;
use std::time::Duration;

use crate::components::auth::use_auth;
use crate::components::icons::IconChevronDown;
use crate::components::icons::IconLoader;
use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::dialog::*;
use crate::components::ui::dropwdown::*;
use crate::components::ui::input::*;
use crate::components::ui::label::*;
use capi_primitives::menu::MenuAlign;
use capi_primitives::menu::MenuSide;

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

#[component]
pub fn CreateChannelDialog(
    open: RwSignal<bool>,
    #[prop(into)] server: Signal<Option<Server>>,
    categories: Signal<Option<Vec<Category>>>,
    #[prop(into)] category: Option<Category>,
) -> impl IntoView {
    let create_channel = UseMutation::new::<CreateChannel>();
    let (name, set_name) = signal(String::default());
    let selected_category = RwSignal::new(category.clone());
    let pending = create_channel.pending();
    let auth = use_auth().auth();

    let (show_success_message, set_show_success_message) = signal(false);

    Effect::new(move |_| {
        if create_channel.value().get().is_some() {
            set_show_success_message(true);
            set_timeout(
                move || {
                    open.set(false);
                },
                Duration::from_millis(300),
            );
        }
    });

    view! {
        <Dialog
            open=open
            on_open_change=Callback::new(move |open_state: bool| {
                if !open_state {
                    set_name("".to_string());
                    selected_category.set(category.clone());
                    set_show_success_message.set(false);
                }
            })
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
                    <Button
                        class="w-full relative overflow-hidden"
                        variant=ButtonVariants::Secondary
                        size=ButtonSizes::Sm
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(server) = server.get() {
                                    if let Some(user) = auth.get().and_then(|res|res.ok()).flatten() {
                                        let input = CreateChannel { name: name.get(), server: server.id , category: selected_category.get().map(|category| category.id), auth: user.id };
                                        create_channel.dispatch(input);
                                    }
                                }
                            }
                        }
                        disabled=Signal::derive(move || pending.get() || server.get().is_none() || show_success_message.get() || name.get().is_empty())
                    >
                        {move || {
                            let is_success = show_success_message.get();
                            let is_pending = pending.get();

                            view! {
                                <div class="absolute inset-0 flex items-center justify-center transition-all duration-300 ease-in-out-expo"
                                    class:translate-y-0=!is_success && !is_pending
                                    class:opacity-100=!is_success && !is_pending
                                    class:translate-y-full=is_success || is_pending
                                    class:opacity-0=is_success || is_pending
                                >
                                    "Create"
                                </div>

                                <div class="absolute inset-0 flex items-center justify-center transition-all duration-300 ease-in-out-expo"
                                    class:translate-y-0=is_pending
                                    class:opacity-100=is_pending
                                    class:-translate-y-full=!is_pending && !is_success
                                    class:translate-y-full=!is_pending && is_success
                                    class:opacity-0=!is_pending
                                >
                                    <IconLoader class="animate-spin text-lg" />
                                </div>

                                <div class="absolute inset-0 flex items-center justify-center transition-all duration-300 ease-in-out-expo"
                                    class:translate-y-0=is_success
                                    class:opacity-100=is_success
                                    class:-translate-y-full=!is_success
                                    class:opacity-0=!is_success
                                >
                                    <span class="text-sm">
                                        {format!("{} created", name.get())}
                                    </span>
                                </div>
                            }
                        }}
                    </Button>
                </DialogFooter>
            </DialogPopup>
        </Dialog>

    }
}
