use crate::components::auth::use_auth;
use crate::components::icons::IconImage;
use crate::components::icons::IconX;
use crate::components::primitives::tooltip::ToolTipSide;
use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::tooltip::*;
use crate::components::uploadthing::upload_file;
use crate::components::uploadthing::UploadResult;
use api::files::RemoveServerBanner;
use api::files::RemoveServerImage;
use api::files::SetServerImageUrl;
use convex_client::leptos::Mutation;
use web_sys::HtmlInputElement;

use super::Title;
use api::files::GenerateUploadUrl;
use api::files::SetServerBannerUrl;
use common::convex::Server;
use convex_client::leptos::UseMutation;
use gloo_file::File;
use leptos::prelude::*;

#[component]
pub fn Profile(server: Signal<Option<Server>>) -> impl IntoView {
    view! {
        <Title>
            "Profile"
        </Title>
        <div class="flex flex-col items-center">
            <ProfileBanner server=server>
                <ProfileImageSetting server=server />
            </ProfileBanner>
        </div>
    }
}

#[component]
pub fn ProfileBanner(children: Children, server: Signal<Option<Server>>) -> impl IntoView {
    let auth = use_auth().auth();
    let input_ref = NodeRef::new();

    let set_server_banner = UseMutation::with_local_fn::<File, _, _, _>(move |(file, client)| {
        let auth = auth.get();
        let mut client_mut = client.to_owned();
        let file = file.to_owned();
        let server = server.get();
        async move {
            if let (Some(Ok(Some(auth_data))), Some(server)) = (auth, server) {
                let upload_url = GenerateUploadUrl { auth: auth_data.id };
                let url = upload_url.run(&mut client_mut).await;
                if let Ok(Some(url)) = url {
                    if let Ok(UploadResult { storage_id }) = upload_file(&file, url).await {
                        let set_image = SetServerBannerUrl {
                            auth: auth_data.id,
                            storage: storage_id,
                            server: server.id,
                        };
                        let _ = set_image.run(&mut client_mut).await;
                    }
                }
            }
        }
    });

    let set_server_banner = Callback::new(move |file: File| {
        set_server_banner.dispatch_local(file.clone());
    });

    let remove_user_banner = UseMutation::new::<RemoveServerBanner>();

    let remove_user_banner = Callback::new(move |()| {
        if let (Some(Ok(Some(auth))), Some(server)) = (auth.get(), server.get()) {
            remove_user_banner.dispatch(RemoveServerBanner {
                auth: auth.id,
                server: server.id,
            });
        }
    });

    let handle_file_change = move |ev: web_sys::Event| {
        let target = event_target::<HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                set_server_banner.run(file.into());
            }
        }
    };
    let tooltip_text = RwSignal::new("Edit your banner");

    view! {
        <Avatar class="flex relative bg-muted size-auto items-center justify-center rounded-lg mt-4 w-full h-28">
            <AvatarImage url=MaybeProp::derive(move || server.get().and_then(|server| server.banner_url)) />
            <AvatarFallback >
                <div/>
            </AvatarFallback>

            <ToolTip>
                <ToolTipTrigger class="absolute inset-0 group flex items-center justify-center rounded-lg">
                    <input
                        type="file"
                        accept="image/*"
                        node_ref=input_ref
                        on:change=handle_file_change
                        class="hidden"
                    />
                    {
                        move || {
                            if server.get().and_then(|server| server.image_url).is_none() {
                                view! {
                                    <Button class="absolute right-2 top-2" size=ButtonSizes::Sm variant=ButtonVariants::Outline
                                        on:click=move |_| { if let Some(input) = input_ref.get() { input.click(); } }
                                    >
                                        <IconImage />
                                        "Add cover image"
                                    </Button>
                                }.into_any()
                            } else {
                                view! {
                                    <div
                                        class="absolute inset-0 cursor-pointer group-hover:bg-black/50 transition-colors flex items-center justify-center"
                                        on:click=move |_| { if let Some(input) = input_ref.get() { input.click(); } }
                                        on:pointerenter=move |_| { tooltip_text.set("Change cover image"); }
                                    >
                                        <IconImage class="size-6 text-white opacity-0 group-hover:opacity-100 transition-opacity"/>
                                    </div>
                                    <Button
                                        size=ButtonSizes::Icon
                                        variant=ButtonVariants::Outline
                                        class="absolute opacity-0 size-7 group-hover:opacity-100 transition-opacity ease-out top-2 right-2 rounded-md p-0"
                                        on:pointerenter=move |_| {
                                            tooltip_text.set("Remove cover image");
                                        }
                                        on:click=move |_| {
                                            remove_user_banner.run(());
                                        }
                                    >
                                        <IconX/>
                                    </Button >
                                }.into_any()
                            }
                        }
                    }
                </ToolTipTrigger>
                <ToolTipContent side_of_set=10.0 side=ToolTipSide::Bottom>
                    {move || {
                        tooltip_text.get()
                    }}
                </ToolTipContent>
            </ToolTip>

            {children()}
        </Avatar>
    }
}

#[component]
pub fn ProfileImageSetting(server: Signal<Option<Server>>) -> impl IntoView {
    let input_ref = NodeRef::new();

    let auth = use_auth().auth();

    let set_server_image = UseMutation::with_local_fn::<File, _, _, _>(move |(file, client)| {
        let auth = auth.get();
        let mut client_mut = client.to_owned();
        let file = file.to_owned();
        let server = server.get();
        async move {
            if let (Some(Ok(Some(auth_data))), Some(server)) = (auth, server) {
                let upload_url = GenerateUploadUrl { auth: auth_data.id };
                let url = upload_url.run(&mut client_mut).await;
                if let Ok(Some(url)) = url {
                    if let Ok(UploadResult { storage_id }) = upload_file(&file, url).await {
                        let set_image = SetServerImageUrl {
                            auth: auth_data.id,
                            storage: storage_id,
                            server: server.id,
                        };
                        let _ = set_image.run(&mut client_mut).await;
                    }
                }
            }
        }
    });

    let set_server_image = Callback::new(move |file: File| {
        set_server_image.dispatch_local(file.clone());
    });

    let remove_server_image = UseMutation::new::<RemoveServerImage>();

    let remove_server_image = Callback::new(move |()| {
        if let (Some(Ok(Some(auth))), Some(server)) = (auth.get(), server.get()) {
            remove_server_image.dispatch(RemoveServerImage {
                auth: auth.id,
                server: server.id,
            });
        }
    });

    let handle_file_change = move |ev: web_sys::Event| {
        let target = event_target::<HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                set_server_image.run(file.into());
            }
        }
    };

    let tooltip_text = RwSignal::new("Edit your image");

    view! {
        <Avatar class="absolute shadow-xs left-2 aspect-square bg-background p-1 size-24 rounded-lg overflow-visible">
            <ToolTip>
                <ToolTipTrigger class="absolute inset-1 hover:bg-accent/50 group flex items-center justify-center rounded-lg">
                    <input
                        type="file"
                        accept="image/*"
                        node_ref=input_ref
                        on:change=handle_file_change
                        class="hidden"
                    />
                    <Button
                        size=ButtonSizes::Icon
                        variant=ButtonVariants::Outline
                        class="absolute opacity-0 size-5 group-hover:opacity-100 transition-opacity ease-out top-0 right-0 -translate-y-1/2 translate-x-1/2 rounded-md p-0"
                        on:pointerenter=move |_| {
                            tooltip_text.set("Remove your image");
                        }
                        on:click=move |_| {
                            remove_server_image.run(());
                        }
                    >
                        <IconX/>
                    </Button >
                    <div
                        on:pointerenter=move |_| {
                            tooltip_text.set("Edit your image");
                        }
                        on:click=move |_| {
                            if let Some(input) = input_ref.get() {
                                input.click();
                            }
                        }
                        class="absolute inset-0 cursor-pointer"
                    ></div>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=10.0 side=ToolTipSide::Bottom>
                    {move || {
                        tooltip_text.get()
                    }}
                </ToolTipContent>
            </ToolTip>
            <AvatarImage url=MaybeProp::derive(move || server.get().and_then(|server| server.image_url)) class="rounded-md"/>
            <AvatarFallback class="rounded-lg text-xl">
                {move || server.get().map(|server| server.name.chars().next())}
            </AvatarFallback>
        </Avatar>
    }
}
