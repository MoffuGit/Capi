use common::convex::Server;
use convex_client::leptos::{Mutation, UseMutation};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::auth::use_auth;
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::dialog::{Dialog, DialogFooter, DialogHeader, DialogPopup};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;

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
pub fn CreateCategoryDialog(open: RwSignal<bool>, server: Signal<Option<Server>>) -> impl IntoView {
    let create_category = UseMutation::new::<CreateCategory>();
    let (name, set_name) = signal(String::default());
    let pending = create_category.pending();
    let auth = use_auth().auth();

    // Effect to close the dialog and reset the input after the mutation completes
    Effect::new(move |_| {
        if create_category.value().get().is_some() {
            open.set(false);
            set_name("".to_string());
        }
    });

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
