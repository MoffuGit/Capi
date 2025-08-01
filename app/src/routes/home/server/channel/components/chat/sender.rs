use crate::components::icons::{IconPaperClip, IconSend, IconSticker, IconTrash, IconX};
use crate::components::primitives::collapsible::use_transition_status;
use crate::components::primitives::common::status::TransitionStatus;
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::routes::server::channel::components::chat::ChatContext;
use common::convex::{Channel, ChannelMessage, FileMetaData, Member};
use convex_client::leptos::{Mutation, UseMutation};
use gloo_file::File;
use leptos::html::{Div, Input};
use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::JsCast as _;
use web_sys::{Event, HtmlInputElement};

#[component]
pub fn Attachment(
    attachment: FileMetaData,
    idx: usize,
    attachments: RwSignal<Vec<FileMetaData>>,
) -> impl IntoView {
    let FileMetaData {
        id,
        creation_time,
        content_type,
        sha256,
        size,
    } = attachment;
    view! {
        <div class="relative w-40 h-40 p-2 rounded-lg border border-base-100 flex flex-col items-center justify-around">
            <Button
                size=ButtonSizes::Icon variant=ButtonVariants::Destructive
                on:click=move |_| {
                    attachments.update(|attachments| {
                        attachments.remove(idx);
                    });
                }
                class="absolute top-1 right-1"
            >
                <IconTrash
                />
            </Button>
            <div class="w-full text-start max-h-4 text-xs text-nowrap truncate inline-block">
                // {name}
            </div>
        </div>

    }
}

#[component]
pub fn MsgRefDisplay(
    #[prop(into)] msg_ref: Signal<Option<ChannelMessage>>,
    on_clear_ref: Callback<()>,
) -> impl IntoView {
    let context: ChatContext = use_context().expect("should return teh chat context");
    let cached_members = context.cached_members;
    let cached_member = Memo::new(move |prev| {
        if let Some(msg) = msg_ref.get() {
            cached_members
                .get()
                .and_then(|members| members.get(&msg.sender).map(|member| member.name.clone()))
        } else {
            prev.flatten().cloned()
        }
    });
    let open = Signal::derive(move || msg_ref.get().is_some());
    let state = use_transition_status(open, true, true, 150, 150);
    view! {
        <Show when=move || state.mounted.get()>
            <div class="w-full h-auto overflow-hidden">
                <div data-state=move || {
                        match state.transition_status.get() {
                            TransitionStatus::Starting => "opening",
                            TransitionStatus::Ending => "closing",
                            TransitionStatus::Idle => "open",
                            TransitionStatus::Undefined => "closed",
                        }
                    }
                    class="relative bg-background w-full px-3 py-2 text-sm flex items-center rounded-t-md border-t border-t-input border-l border-l-input border-r border-r-input transition-[opacity,translate] duration-150 ease-out-quad justify-between data-[state=open]:opacity-100 data-[state=closing]:opacity-0 data-[state=closed]:opacity-0 data-[state=closing]:translate-y-full data-[state=closed]:translate-y-full"
                >
                    <div class="flex flex-col text-xs text-base-content/70 truncate">
                        <div class="text-xs">
                            <span class="text-muted-foreground">
                                 "Replying to "
                            </span>
                            <span class="font-medium text-base-content">
                                {move || {
                                    cached_member.get()
                                }}
                            </span>
                        </div>
                    </div>
                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="size-6" on:click=move |_| on_clear_ref.run(())>
                        <IconX />
                    </Button>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn AttachmentPreviewList(attachments: RwSignal<Vec<FileMetaData>>) -> impl IntoView {
    view! {
        <Show when=move || !attachments.get().is_empty()>
            <div class="relative w-full h-auto  border first:rounded-t-lg border-b-0 border-base-100 flex items-center p-2 text-sm">
                {
                    move || {
                        attachments.get().iter().enumerate().map(|(idx, att)| {
                            view!{
                                <Attachment attachment=att.clone() idx=idx attachments=attachments/>
                            }
                        }).collect_view()
                    }
                }
            </div>
        </Show>
    }
}

#[component]
pub fn MessageInputArea(
    message: RwSignal<String>,
    content_ref: NodeRef<Div>,
    #[prop(into)] channel_name: Signal<Option<String>>,
) -> impl IntoView {
    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
        }
    };

    view! {
        <div class="relative self-center h-fit w-full overflow-y-auto overflow-x-hidden ">
            <div class="text-sm font-normal relative mx-2">
                <div>
                    <Show when=move || message.get().is_empty()>
                        <div class="absolute left-0 select-none text-muted-foreground">
                            "Send a message"
                        </div>
                    </Show>
                </div>
                <div
                    on:input=on_input
                    node_ref=content_ref
                    class="relative outline-0 wrap-break-word text-left whitespace-break-spaces"
                    contenteditable="true"
                    aria-multiline="true"
                    spellcheck="true"
                    aria-invalid="false">
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn MessageActionButtons(
    on_send: Callback<()>,
    attachments: RwSignal<Vec<FileMetaData>>,
) -> impl IntoView {
    let file_input_ref: NodeRef<Input> = NodeRef::new();

    let on_file_selected = move |event: Event| {
        let input = event.target().unwrap().unchecked_into::<HtmlInputElement>();
        if let Some(files) = input.files() {
            let mut new_attachments: Vec<FileMetaData> = Vec::new();
            for i in 0..files.length() {
                if let Some(file) = files.get(i) {
                    let gloo_file = File::from(file);
                    // new_attachments.push(FileMetaData {
                    //     id: Ulid::new().to_string(), // Client-side unique ID
                    //     file_type: gloo_file.raw_type(),
                    //     size: gloo_file.size() as usize,
                    //     url: None, // URL will be set after actual upload to a storage service
                    // });
                }
            }
            attachments.update(|current_attachments| {
                current_attachments.extend(new_attachments);
            });
            input.set_value("");
        }
    };
    view! {
        <input type="file" multiple=true class="hidden" node_ref=file_input_ref on:change=on_file_selected />
        <div class="flex items-center gap-3 p-1">
            <Button
                on:click=move |_| {
                    if let Some(input) = file_input_ref.get() {
                        input.click();
                    }
                }
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="size-6 text-muted-foreground hover:text-foreground">
                <IconPaperClip/>
            </Button>
            <Button
                size=ButtonSizes::Icon variant=ButtonVariants::Ghost
                class="size-6 text-muted-foreground hover:text-foreground"
            >
                <IconSticker/>
            </Button>
            <Button size=ButtonSizes::Icon
                variant=ButtonVariants::Secondary
                class="size-6 text-muted-foreground hover:text-foreground"
                on:click=move |_| on_send.run(())
            >
                <IconSend/>
            </Button>
        </div>
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SendMessage {
    #[serde(rename = "channelId")]
    channel: String,
    content: String,
    #[serde(rename = "senderId")]
    sender: String,
    #[serde(rename = "referenceId")]
    reference: Option<String>,
}

impl Mutation for SendMessage {
    type Output = String;

    fn name(&self) -> String {
        "messages:createMessage".into()
    }
}

#[component]
pub fn Sender(
    channel: Signal<Option<Channel>>,
    member: Signal<Option<Member>>,
    sender_ref: NodeRef<Div>,
) -> impl IntoView {
    let send = UseMutation::new::<SendMessage>();
    let message = RwSignal::new(String::default());
    let content_ref: NodeRef<Div> = NodeRef::new();

    let context: ChatContext = use_context().expect("should access the chat context");

    let attachments = context.attachments;
    let msg_ref = context.msg_reference;

    let on_clear_msg_ref = Callback::new(move |_| {
        msg_ref.set(None);
    });

    let on_send_message = Callback::new(move |_| {
        if let Some(channel_data) = channel.get() {
            if let Some(member_data) = member.get() {
                let current_message_content = message.get();
                if !current_message_content.is_empty() || !attachments.get().is_empty() {
                    let msg_ref_id = msg_ref.get().map(|m| m.id);
                    send.dispatch(SendMessage {
                        channel: channel_data.id,
                        content: current_message_content,
                        sender: member_data.id,
                        reference: msg_ref_id,
                    });
                    // Clear the message input and message reference after sending
                    message.set(String::default());
                    if let Some(div) = content_ref.get() {
                        div.set_inner_text("");
                    }
                    msg_ref.set(None);
                }
            }
        }
    });

    Effect::watch(
        move || send.value().get(),
        move |message_id, _, _| {
            // Original attachment sending logic (commented out in source)
            // if let Some(Ok(message_id)) = message_id {
            //     if !attachments.get().is_empty() {
            //         let multipart = FormData::new().expect("should create the form data");
            //         multipart
            //             .append_with_str("message_id", &message_id.to_string())
            //             .expect("Something");
            //         for attachment in attachments.get() {
            //             let file_name = attachment.data.name;
            //             multipart
            //                 .append_with_blob_and_filename(
            //                     &file_name,
            //                     &Blob::new_with_options(
            //                         &*attachment.chunks,
            //                         Some(&attachment.data.file_type),
            //                     )
            //                     .into(),
            //                     &file_name,
            //                 )
            //                 .expect("should add the file to the form data")
            //         }
            //         attachments.set(vec![]);
            //         send_attachments.dispatch_local(multipart);
            //     }
            // }
        },
        false,
    );

    view! {
        <div class="w-full absolute bottom-0 bg-transparent flex flex-col z-20 isolate" node_ref=sender_ref>
            <div class="max-h-96 w-full flex flex-col items-center justify-center px-5 text-base shadow-xs md:text-sm">
                <MsgRefDisplay msg_ref=msg_ref on_clear_ref=on_clear_msg_ref/>
                // <AttachmentPreviewList attachments=attachments/>
                <div class="flex w-full px-3 py-2 justify-between border-input only:rounded-md rounded-b-md transition-all duration-150 ease-out border bg-background">
                    <MessageInputArea
                        message=message
                        content_ref=content_ref
                        channel_name=Signal::derive(move || channel.get().map(|c| c.name))
                    />
                    <MessageActionButtons
                        on_send=on_send_message
                        attachments=attachments
                    />
                </div>
            </div>
            <div class="absolute inset-0 mb-5 bg-gradient-to-b -z-10 from-transparent to-background"/>
            <div class="bg-background w-full h-5"/>
        </div>
    }
}
