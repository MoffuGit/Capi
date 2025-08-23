use crate::components::auth::use_auth;
use crate::components::uploadthing::{upload_file, UploadResult};
use crate::routes::home::components::user_settings::content::{
    Setting, SettingAction, SettingData, SettingDescription, SettingTitle,
};
use crate::routes::use_profile;
use capi_ui::avatar::*;
use capi_ui::button::*;
use capi_ui::input::*;
use capi_ui::tabs::*;
use capi_ui::tooltip::*;
use convex_client::leptos::Mutation;
use gloo_file::File;
use icons::{IconImage, IconX};
use web_sys::HtmlInputElement;

use super::Title;
use api::files::{GenerateUploadUrl, RemoveUserBanner, RemoveUserImage, SetBannerUrl, SetImageUrl};
use convex_client::leptos::UseMutation;
use leptos::prelude::*;

#[component]
pub fn Profiles() -> impl IntoView {
    view! {
        <Title>
            "Profiles"
        </Title>
        <Tabs tab=RwSignal::new("main".to_string())>
            <TabsList>
                <Tab
                    class="z-20"
                    value="main"
                >
                    "Account"
                </Tab>
                <Tab
                    class="z-20"
                    value="member"
                >
                    "Members"
                </Tab>
                <TabIndicator class="z-10"/>
            </TabsList>
            <TabPanel value="main">
                <Profile/>
            </TabPanel>
            <TabPanel value="member">
                "Member's Profile"
            </TabPanel>
        </Tabs>
    }
}

#[component]
pub fn Profile() -> impl IntoView {
    let user = use_profile();
    let auth = use_auth().auth();

    let set_user_image = UseMutation::with_local_fn::<File, _, _, _>(move |(file, client)| {
        let auth = auth.get();
        let mut client_mut = client.to_owned();
        let file = file.to_owned();
        async move {
            if let Some(Ok(Some(auth_data))) = auth {
                let upload_url = GenerateUploadUrl { auth: auth_data.id };
                let url = upload_url.run(&mut client_mut).await;
                if let Ok(Some(url)) = url {
                    if let Ok(UploadResult { storage_id }) = upload_file(&file, url).await {
                        let set_image = SetImageUrl {
                            auth: auth_data.id,
                            storage_id,
                        };
                        let _ = set_image.run(&mut client_mut).await;
                    }
                }
            }
        }
    });

    let set_user_image = Callback::new(move |file: File| {
        set_user_image.dispatch_local(file.clone());
    });

    let remove_user_image = UseMutation::new::<RemoveUserImage>();

    let remove_user_image = Callback::new(move |_| {
        if let Some(Ok(Some(auth))) = auth.get() {
            remove_user_image.dispatch(RemoveUserImage { auth: auth.id });
        }
    });

    view! {
        {
            move || {
                user.get().map(|user| {
                    view!{
                        <div class="flex flex-col items-center">
                            <ProfileBanner banner_url=user.banner_url>
                                <ProfileImageSetting
                                    image_url=user.image_url
                                    on_file_selected=set_user_image
                                    remove_user_image=remove_user_image
                                />
                            </ProfileBanner>
                        </div>
                        <ProfileUsernameSetting />
                        <ProfileAboutSetting />
                    }
                })
            }
        }
    }
}

#[component]
pub fn ProfileBanner(
    #[prop(into)] banner_url: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let auth = use_auth().auth();
    let input_ref = NodeRef::new();

    let set_user_banner = UseMutation::with_local_fn::<File, _, _, _>(move |(file, client)| {
        let auth = auth.get();
        let mut client_mut = client.to_owned();
        let file = file.to_owned();
        async move {
            if let Some(Ok(Some(auth_data))) = auth {
                let upload_url = GenerateUploadUrl { auth: auth_data.id };
                let url = upload_url.run(&mut client_mut).await;
                if let Ok(Some(url)) = url {
                    if let Ok(UploadResult { storage_id }) = upload_file(&file, url).await {
                        let set_image = SetBannerUrl {
                            auth: auth_data.id,
                            storage_id,
                        };
                        let _ = set_image.run(&mut client_mut).await;
                    }
                }
            }
        }
    });

    let set_user_banner = Callback::new(move |file: File| {
        set_user_banner.dispatch_local(file.clone());
    });

    let remove_user_banner = UseMutation::new::<RemoveUserBanner>();

    let remove_user_banner = Callback::new(move |()| {
        if let Some(Ok(Some(auth))) = auth.get() {
            remove_user_banner.dispatch(RemoveUserBanner { auth: auth.id });
        }
    });

    let handle_file_change = move |ev: web_sys::Event| {
        let target = event_target::<HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                set_user_banner.run(file.into());
            }
        }
    };

    let tooltip_text = RwSignal::new("Edit your banner");

    view! {
        <Avatar class="flex relative bg-muted size-auto items-center justify-center rounded-lg mt-4 w-full h-28">
            <AvatarImage url=banner_url />
            <AvatarFallback >
                <div/>
            </AvatarFallback>

            <ToolTip>
                <ToolTipTrigger class="absolute inset-0 group flex items-center justify-center rounded-lg">
                    // Hidden input for file selection
                    <input
                        type="file"
                        accept="image/*"
                        node_ref=input_ref
                        on:change=handle_file_change
                        class="hidden"
                    />

                    {
                        move || {
                            if banner_url.get().is_none() {
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
                                        size=ButtonSizes::IconXs
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
pub fn ProfileImageSetting(
    #[prop(into)] image_url: MaybeProp<String>,
    on_file_selected: Callback<File>,
    remove_user_image: Callback<()>,
) -> impl IntoView {
    let input_ref = NodeRef::new();

    let handle_file_change = move |ev: web_sys::Event| {
        let target = event_target::<HtmlInputElement>(&ev);
        if let Some(file_list) = target.files() {
            if let Some(file) = file_list.get(0) {
                on_file_selected.run(file.into());
            }
        }
    };

    let tooltip_text = RwSignal::new("Edit your image");

    view! {
        <Avatar class="absolute shadow-xs left-2 aspect-square bg-background p-1 size-24 rounded-lg overflow-visible">
            <ToolTip>
                <ToolTipTrigger class="absolute inset-1 hover:bg-accent/50 group flex items-center justify-center rounded-lg">
                    // Hidden input for file selection
                    <input
                        type="file"
                        accept="image/*"
                        node_ref=input_ref
                        on:change=handle_file_change
                        class="hidden"
                    />
                    <Button
                        size=ButtonSizes::IconXs
                        variant=ButtonVariants::Outline
                        class="absolute opacity-0 size-5 group-hover:opacity-100 transition-opacity ease-out top-0 right-0 -translate-y-1/2 translate-x-1/2 rounded-md p-0"
                        on:pointerenter=move |_| {
                            tooltip_text.set("Remove your image");
                        }
                        on:click=move |_| {
                            remove_user_image.run(());
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
            <AvatarImage url=image_url class="rounded-md"/>
            <AvatarFallback class="rounded-lg">
                <div/>
            </AvatarFallback>
        </Avatar>
    }
}

#[component]
pub fn ProfileUsernameSetting() -> impl IntoView {
    view! {
        <Setting class="mt-6">
            <SettingData>
                <SettingTitle>
                    "Username"
                </SettingTitle>
                <SettingDescription>
                    "A unique name for your profile"
                </SettingDescription>
            </SettingData>
            <SettingAction>
                // TODO: Add actual value binding and on_input handler for saving
                <Input />
            </SettingAction>
        </Setting>
    }
}

#[component]
pub fn ProfileAboutSetting() -> impl IntoView {
    view! {
        <Setting>
            <SettingData>
                <SettingTitle>
                    "About you"
                </SettingTitle>
                <SettingDescription>
                    "Write a description for your profile"
                </SettingDescription>
            </SettingData>
            <SettingAction>
                // TODO: Add actual value binding and on_input handler for saving
                <Input />
            </SettingAction>
        </Setting>
    }
}
