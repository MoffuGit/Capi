use std::str::FromStr;

use crate::components::icons::{IconCirclePlus, IconSend, IconSticker, IconTrash};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::uploadthing::input::FileInput;
use crate::routes::server::channel::components::chat::ChatContext;
use common::convex::{Channel, Member};
use convex_client::leptos::{Mutation, UseMutation};
use gloo_file::Blob;
use leptos::html::Div;
use leptos::prelude::*;
use serde::Serialize;
use uploadthing::{FileData, FileType, UploadthingFile};
use web_sys::{FormData, Url};

#[component]
pub fn Attachment(
    attachment: UploadthingFile,
    idx: usize,
    attachments: RwSignal<Vec<UploadthingFile>>,
) -> impl IntoView {
    let FileData {
        name, file_type, ..
    } = attachment.data;
    let file_type = FileType::from_str(&file_type).unwrap();
    let url: RwSignal<Option<String>> = RwSignal::new(match file_type {
        FileType::Jpeg | FileType::Png => {
            Url::create_object_url_with_blob(&Blob::new(&*attachment.chunks).into()).ok()
        }
        _ => None,
    });
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
            {
                match file_type {
                    FileType::Jpeg | FileType::Png =>  {
                        view! {
                            <img
                                class="w-full mx-1 h-28 object-cover rounded-md"
                                src=move || url.get().unwrap()
                                on:load=move |_| {
                                    let _ = Url::revoke_object_url(&url.get().unwrap());
                                }
                            />
                        }.into_any()
                    },
                    _ => {
                        ().into_any()
                    }
                }
            }
            <div class="w-full text-start max-h-4 text-xs text-nowrap truncate inline-block">
                {name}
            </div>
        </div>

    }
}

#[derive(Debug, Serialize, Clone)]
pub struct SendMessage {
    channel: String,
    message: String,
    member: String,
}

impl Mutation for SendMessage {
    type Output = String;

    fn name(&self) -> String {
        "messages:createMessage".into()
    }
}

#[component]
pub fn Sender(channel: Signal<Option<Channel>>, member: Signal<Option<Member>>) -> impl IntoView {
    let send = UseMutation::new();
    let message = RwSignal::new(String::default());
    let content_ref: NodeRef<Div> = NodeRef::new();
    let on_input = move |_| {
        if let Some(div) = content_ref.get() {
            message.set(div.inner_text());
        }
    };

    // let send_attachments = Action::new_local(|data: &FormData| {
    //     let data = data.clone();
    //     send_message_attachments(data.into())
    // });

    let context: ChatContext = use_context().expect("should acces the chat context");

    let attachments = context.attachments;

    Effect::watch(
        move || send.value().get(),
        move |message_id, _, _| {
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
        <div class="flex flex-col gap-2 p-5">
            <Show when=move || !context.attachments.get().is_empty()>
                <div class="relative w-full h-auto bg-base-300 border first:rounded-t-lg border-b-0 border-base-100 flex items-center p-2 text-sm">
                    {
                        move || {
                            context.attachments.get().iter().enumerate().map(|(idx, att)| {
                                view!{
                                    <Attachment attachment=att.clone() idx=idx attachments=context.attachments/>
                                }
                            }).collect_view()
                        }
                    }
                </div>
            </Show>
            <div class="border-input dark:bg-input/30 flex w-full rounded-md border bg-transparent px-3 py-2 text-base shadow-xs md:text-sm justify-between">

                <div class="flex items-center justify-center">
                    <Button size=ButtonSizes::Icon variant=ButtonVariants::Ghost class="relative">
                        <FileInput class="absolute inset-0 opacity-0" files=context.attachments/>
                        <IconCirclePlus/>
                    </Button>
                </div>
                <div class="relative self-center h-fit w-full" /* style=move || format!("height: {}px", height.get()) */>
                    <div class="text-sm font-normal relative mx-2">
                        <div>
                            <Show when=move || message.get().is_empty()>
                                <div class="absolute left-0 select-none text-base-content/40">
                                    {move || channel.get().map(|channel| format!("Message #{}", channel.name))}
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

                <div class="flex items-center gap-2">
                    <Button
                        size=ButtonSizes::Icon variant=ButtonVariants::Ghost
                    >
                        <IconSticker/>
                    </Button>
                    <Button size=ButtonSizes::Icon
                        on:click=move |_| {
                            if let Some(channel) = channel.get() {
                                if let Some(member) = member.get() {
                                    send.dispatch(SendMessage {  channel: channel.id, message: message.get(), member: member.id });
                                }
                            }
                        }
                    >
                        <IconSend/>
                    </Button>
                </div>
            </div>
        </div>

    }
}
