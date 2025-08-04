use api::files::GenerateUploadUrl;
use chrono::Utc;
use common::files::{read_file, ClientFile};
use convex_client::leptos::{Mutation, UseMutation};
use gloo_file::File;
use leptos::svg::image;
use leptos::task::spawn_local;
use leptos::{html, prelude::*};
use leptos_router::components::A;
use serde::Serialize;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::components::auth::use_auth;
use crate::components::icons::{
    IconCirclePlus, IconCompass, IconGlobe, IconImage, IconInbox, IconMessageCircle, IconPencil,
    IconSearch, IconX,
};
use crate::components::primitives::menu::{MenuAlign, MenuSide};
use crate::components::ui::avatar::*;
use crate::components::ui::button::*;
use crate::components::ui::context::*;
use crate::components::ui::dialog::*;
use crate::components::ui::input::*;
use crate::components::ui::label::*;
use crate::components::ui::sidebar::*;
use crate::components::ui::tooltip::*;
use crate::components::uploadthing::{upload_file, UploadResult};
use crate::routes::home::components::servers::ServersItems;
use crate::routes::use_profile;

use super::sidebar::{ServerData, SideBarOption};

#[derive(Debug, Serialize, Clone)]
struct JoinWithInvitation {
    invitation: String,
    user: String,
}

impl Mutation for JoinWithInvitation {
    type Output = Option<String>;

    fn name(&self) -> String {
        "invitations:joinServerWithInvitation".into()
    }
}

#[component]
pub fn SidebarIcons(
    data: Signal<Option<Vec<ServerData>>>,
    option: RwSignal<Option<SideBarOption>>,
) -> impl IntoView {
    let set_option = Callback::new(move |_| {
        option.set(None);
    });
    view! {
        <Sidebar
            collapsible=SideBarCollapsible::None
            class="w-[calc(var(--sidebar-width-icon)+1px)]! border-r"
        >
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupContent class="px-1.5 md:px-0">
                        <SidebarMenu>
                            <Direct set_option=set_option/>
                            <InboxOption option=option/>
                            <SearchOption option=option/>
                            <ServerMenu set_option=set_option/>
                            <SidebarSeparator
                                class="mr-2 data-[orientation=horizontal]:w-4 my-0.5"
                            />
                            <ServersItems data=data set_option=set_option/>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    }
}

#[component]
pub fn Direct(set_option: Callback<()>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <A href="/servers/me"
                        {..}
                        on:click=move |_| set_option.run(())
                    >
                        <SidebarMenuButton
                            size=crate::components::ui::sidebar::SidebarMenuButtonSize::Sm
                            class="md:h-8 md:p-0 flex items-center justify-center"
                        >
                            <IconMessageCircle class="size-4 text-sidebar-foreground/70" />
                        </SidebarMenuButton>
                    </A>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0>
                    "Direct Messages"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

    }
}

#[component]
pub fn InboxOption(option: RwSignal<Option<SideBarOption>>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <SidebarMenuButton
                        class="px-2.5 md:px-2"
                        {..}
                        on:click=move |_| {
                            option.set(Some(SideBarOption::Inbox))
                        }
                    >
                        <IconInbox class="size-4 text-sidebar-foreground/70"/>
                    </SidebarMenuButton>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0>
                    "Inbox"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

    }
}

#[component]
pub fn SearchOption(option: RwSignal<Option<SideBarOption>>) -> impl IntoView {
    view! {
        <SidebarMenuItem>
            <ToolTip>
                <ToolTipTrigger>
                    <SidebarMenuButton
                        class="px-2.5 md:px-2"
                        {..}
                        on:click=move |_| {
                            option.set(Some(SideBarOption::Search))
                        }
                    >
                        <IconSearch class="size-4 text-sidebar-foreground/70"/>
                    </SidebarMenuButton>
                </ToolTipTrigger>
                <ToolTipContent side_of_set=3.0 >
                    "Search"
                </ToolTipContent>
            </ToolTip>
        </SidebarMenuItem>

    }
}

#[component]
pub fn ServerMenu(set_option: Callback<()>) -> impl IntoView {
    let create_open = RwSignal::new(false);
    let join_open = RwSignal::new(false);
    view! {
        <SidebarMenuItem>
            <ContextMenu>
                <A
                    href="/servers"
                    {..}
                    on:click=move |_| set_option.run(())
                >
                    <ContextMenuTrigger pointer=false >
                        <ToolTip>
                            <ToolTipTrigger>
                            <SidebarMenuButton
                              class="px-2.5 md:px-2"
                            >
                                <IconGlobe class="size-4 text-sidebar-foreground/70"/>
                            </SidebarMenuButton>
                            </ToolTipTrigger>
                            <ToolTipContent >
                                "Servers"
                            </ToolTipContent>
                        </ToolTip>
                    </ContextMenuTrigger>
                </A>
                <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                    <ContextMenuLabel>
                        "Servers"
                    </ContextMenuLabel>
                    <ContextMenuItem
                        {..}
                        on:click=move |_| {
                            create_open.set(true);
                        }
                    >
                        <IconPencil/>
                        "Create"
                    </ContextMenuItem>
                    <ContextMenuItem
                        {..}
                        on:click=move |_| {
                            join_open.set(true);
                        }
                    >
                        <IconCirclePlus/>
                        "Join"
                    </ContextMenuItem>
                    <A
                        href="/servers/#discover"
                        on:click=move |_| {
                            set_option.run(())
                        }
                    >
                        <ContextMenuItem close_on_click=true>
                            <IconCompass />
                            "Search" // This "Search" is for Discover servers, not the global search
                        </ContextMenuItem>
                    </A>
                </ContextMenuContent>
            </ContextMenu>
        </SidebarMenuItem>
        <CreateServerDialog open=create_open/>
        <JoinServerDialog open=join_open/>
    }
}

#[component]
pub fn JoinServerDialog(open: RwSignal<bool>) -> impl IntoView {
    let user = use_profile();
    let join_server = UseMutation::new();
    let (name, set_name) = signal(String::default());
    let pending = join_server.pending();
    view! {
        <Dialog open=open>
            <DialogPopup>
                <DialogHeader>
                    <DialogTitle >"Join Server"</DialogTitle>
                    <DialogDescription>
                        "use your invitation code"
                    </DialogDescription>
                </DialogHeader>
                    <div class="grid gap-2">
                        <Label {..} for="invitation">Invitation Code</Label>
                        <Input
                            {..}
                            id="invitation"
                            type="text"
                            placeholder="Your code"
                            required=true
                            value=name
                            on:input=move |ev| set_name(event_target_value(&ev))
                        />
                    </div>
                <DialogFooter>
                    <div/>
                    <button
                        on:click=move |_| {
                            if !name.get().is_empty() {
                                if let Some(user) = user.get() {
                                    join_server.dispatch(JoinWithInvitation {
                                        invitation: name.get(),
                                        user: user.id
                                    });
                                }
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

#[derive(Debug, Serialize, Clone)]
pub struct CreateServer {
    name: String,
    auth: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage: Option<String>,
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

    let on_file_selected = move |event: Event| {
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        if let Some(files) = input.files() {
            let num_files = files.length();

            spawn_local(async move {
                for i in 0..num_files {
                    if let Some(file) = files.get(i) {
                        read_file(file.into(), move |file| {
                            if let Ok(file) = file {
                                image_file.set(Some(file));
                            }
                        });
                    }
                }
            });
            input.set_value("");
        }
    };

    let on_clear_image = Callback::new(move |()| {
        image_file.set(None);
    });

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
                    };
                    let _ = create_server_input.run(&mut client_mut).await;
                }
            }
        },
    );
    let pending = create_server_action.pending();

    // Effect to close the dialog and reset the input after the mutation completes
    Effect::new(move |_| {
        if create_server_action.value().get().is_some() {
            open.set(false);
            set_name("".to_string());
            image_file.set(None);
        }
    });

    view! {
        <Dialog open=open>
            <DialogPopup>
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
                </div>
                <DialogFooter>
                    <Button
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
                        "Create"
                    </Button>
                </DialogFooter>
            </DialogPopup>
        </Dialog>
    }
}
