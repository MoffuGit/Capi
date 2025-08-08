use std::time::Duration;

use api::files::GenerateUploadUrl;
use chrono::Utc;
use common::convex::ServerType;
use common::files::{read_file, ClientFile};
use convex_client::leptos::{Mutation, UseMutation};
use gloo_file::File;
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::components::auth::use_auth;
use crate::components::icons::{IconImage, IconLoader, IconX};
use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::dialog::*;
use crate::components::ui::input::Input;
use crate::components::ui::label::*;
use crate::components::uploadthing::{upload_file, UploadResult};

#[derive(Debug, Serialize, Clone)]
pub struct CreateServer {
    name: String,
    auth: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<String>,
    #[serde(rename = "type")]
    _type: ServerType,
}

impl Mutation for CreateServer {
    type Output = ();

    fn name(&self) -> String {
        "server:create".into()
    }
}

#[component]
pub fn CreateServerDialog(open: RwSignal<bool>) -> impl IntoView {
    let auth = use_auth().auth();
    let (name, set_name) = signal(String::default());

    let image_file: RwSignal<Option<ClientFile>> = RwSignal::new(None);
    let image_input_ref: NodeRef<html::Input> = NodeRef::new();

    let (success, set_success) = signal(false);

    let on_file_selected = move |event: Event| {
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        if let Some(files) = input.files() {
            spawn_local(async move {
                if let Some(file) = files.get(0) {
                    read_file(file.into(), move |file| {
                        if let Ok(file) = file {
                            image_file.set(Some(file));
                        }
                    });
                }
            });
            input.set_value("");
        }
    };

    let on_clear_image = Callback::new(move |()| {
        image_file.set(None);
    });

    let is_private = RwSignal::new(true);

    let create_server_action = UseMutation::with_local_fn::<(String, Option<File>), _, _, _>(
        move |((server_name, file_opt), client)| {
            let auth = auth.get();
            let server_name = server_name.to_owned();
            let file_opt = file_opt.to_owned();
            let mut client_mut = client.to_owned();
            async move {
                if let Some(Ok(Some(auth_data))) = auth {
                    let mut storage_id: Option<String> = None;
                    if let Some(file) = file_opt {
                        let upload_url = GenerateUploadUrl { auth: auth_data.id };
                        if let Ok(Some(url)) = upload_url.run(&mut client_mut).await {
                            if let Ok(UploadResult {
                                storage_id: uploaded_id,
                            }) = upload_file(&file, url).await
                            {
                                storage_id = Some(uploaded_id);
                            }
                        }
                    }

                    let create_server_input = CreateServer {
                        name: server_name,
                        auth: auth_data.id,
                        storage: storage_id,
                        _type: if is_private.get() {
                            ServerType::Private
                        } else {
                            ServerType::Public
                        },
                    };
                    let _ = create_server_input.run(&mut client_mut).await;
                }
            }
        },
    );

    let pending = create_server_action.pending();

    Effect::new(move |_| {
        if create_server_action.value().get().is_some() {
            set_success(true);
            set_timeout(
                move || {
                    open.set(false);
                },
                Duration::from_millis(1000),
            );
        }
    });

    view! {
        <Dialog
            open=open
            on_open_change=Callback::new(move |open_state: bool| {
                if !open_state {
                    set_success(false);
                    set_name("".to_string());
                    image_file.set(None);
                }
            })
        >
            <DialogPopup class="isolate overflow-hidden">
                <div
                    data-pending=move ||   pending.get().to_string()
                    data-success=move || success.get().to_string()
                    class="flex flex-col duration-[400ms] items-center justify-center -z-10 data-[pending=true]:z-20 data-[success=true]:z-20 data-[success=false]:-translate-y-[32px] data-[success=false]:opacity-0 data-[success=false]:blur-xs transition-all absolute inset-0 w-full h-full gap-2 bg-background rounded-lg ease-out-quad"
                >
                    <Avatar class="relative size-24 rounded-lg bg-muted flex items-center justify-center shadow-sm">
                        <AvatarImage url=MaybeProp::derive(move || image_file.get().map(|file| file.metadata.url))/>
                        <AvatarFallback class="rounded-lg text-4xl capitalize">
                            {
                                move || name.get().chars().next()
                            }
                        </AvatarFallback>
                    </Avatar>
                    <h3 class="text-foreground text-base font-medium capitalize">
                        {
                            move || {
                                format!("Server {} created!", name.get())
                            }
                        }
                    </h3>
                    <p class="text-sm font-normal">
                        "Let's build something great."
                    </p>
                </div>
                <div data-success=move || success.get().to_string() class="grid w-full gap-4 rounded-lg data-[success=true]:translate-y-[8px] data-[success=true]:opacity-0 data-[success=true]:blur-xs duration-[400ms] transition-all ease-out-quad">
                    <DialogHeader>
                        <div class="text-sm flex h-8 items-center px-2">
                            <span class="text-foreground/70">
                                "Create New Server"
                            </span>
                        </div>
                    </DialogHeader>
                    <div class="flex flex-col items-center gap-4">
                        <Avatar class="relative size-24 rounded-lg bg-muted flex items-center justify-center">
                            <AvatarImage url=MaybeProp::derive(move || image_file.get().map(|file| file.metadata.url))/>
                            <AvatarFallback class="rounded-lg text-4xl">
                                <IconImage class="size-10 text-muted-foreground"/>
                            </AvatarFallback>

                            <input
                                type="file"
                                accept="image/*"
                                node_ref=image_input_ref
                                on:change=on_file_selected
                                class="hidden"
                            />
                            {
                                move || {
                                    if image_file.get().is_none() {
                                        view! {
                                            <button
                                                class="absolute inset-0 flex items-center justify-center bg-transparent hover:bg-muted/10 transition-colors rounded-lg"
                                                on:click=move |_| { if let Some(input) = image_input_ref.get() { input.click(); } }
                                            />
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div
                                                class="absolute inset-0 cursor-pointer group-hover:bg-black/50 transition-colors flex items-center justify-center rounded-lg"
                                                on:click=move |_| { if let Some(input) = image_input_ref.get() { input.click(); } }
                                            >
                                                <IconImage class="size-6 text-white opacity-0 group-hover:opacity-100 transition-opacity"/>
                                            </div>
                                            <Button
                                                size=ButtonSizes::Icon
                                                variant=ButtonVariants::Outline
                                                class="absolute opacity-0 size-7 group-hover:opacity-100 transition-opacity ease-out top-2 right-2 rounded-md p-0"
                                                on:click=move |_| { on_clear_image.run(()); }
                                            >
                                                <IconX/>
                                            </Button >
                                        }.into_any()
                                    }
                                }
                            }
                        </Avatar>
                        <div class="grid w-full gap-2">
                            <Label class="px-2" {..} for="server-name">Server Name</Label>
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
                        <Label class="hover:bg-accent/50 active:scale-[.98] transition-[scale] flex items-start gap-3 rounded-lg border w-full p-3 has-[[aria-checked=true]]:border-purple has-[[aria-checked=true]]:bg-purple/10">
                            <Checkbox
                                checked=is_private
                                class="data-[state=checked]:border-purple data-[state=checked]:bg-purple data-[state=checked]:text-white dark:data-[state=checked]:border-purple dark:data-[state=checked]:bg-purple dark:data-[state=checked]:text-white"
                                {..}
                                id="toggle-2"
                            />
                            <div class="grid gap-1.5 font-normal text-muted-foreground peer-data-[state=checked]:text-foreground/70">
                                <p class="text-sm leading-none font-medium text-foreground">
                                    Private Server
                                </p>
                                <p class="text-sm">
                                    Only invited members can join a private server.
                                </p>
                                <p class="text-sm">
                                    You can enable or disable it at any time.
                                </p>
                            </div>
                        </Label>
                    </div>
                    <DialogFooter>
                        <Button
                            class="w-full overflow-hidden relative"
                            variant=ButtonVariants::Secondary
                            size=ButtonSizes::Sm
                            on:click=move |_| {
                                if !name.get().is_empty() {
                                    let gloo_file = image_file.get()
                                        .map(|file| {
                                            File::new_with_options(
                                                &file.metadata.name,
                                                &*file.chunks,
                                                Some(&file.metadata.content_type.to_string()),
                                                Some(Utc::now().into()),
                                            )
                                        });
                                    create_server_action.dispatch_local((name.get(), gloo_file));
                                }
                            }
                            disabled=Signal::derive(move || pending.get() || name.get().is_empty())
                        >
                            <div class="absolute inset-0 flex items-center justify-center transition-all duration-300 ease-in-out-expo"
                                class:translate-y-0=move || !pending.get() && !success()
                                class:opacity-100=move || !pending.get() && !success()
                                class:translate-y-full=move || pending.get() || success()
                                class:opacity-0=move || pending.get() || success()
                            >
                                "Create"
                            </div>

                            <div class="absolute inset-0 flex items-center justify-center transition-all duration-300 ease-in-out-expo"
                                class:translate-y-0=move || pending.get() || success()
                                class:opacity-100=move || pending.get() || success()
                                class:-translate-y-full=move || !pending.get()
                                class:opacity-0=move || !pending.get()
                            >
                                <IconLoader class="animate-spin text-lg" />
                            </div>
                        </Button>
                    </DialogFooter>
                </div>
            </DialogPopup>
        </Dialog>
    }
}
