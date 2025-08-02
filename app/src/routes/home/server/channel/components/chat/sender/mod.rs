mod actions;
mod attachments;
mod input;
mod msg_ref;

use api::files::GenerateUploadUrl;
use chrono::Utc;
use common::convex::{Channel, Member};
use convex_client::leptos::{Mutation, UseMutation};
use gloo_file::File;
use leptos::html::Div;
use leptos::prelude::*;
use serde::Serialize;

use crate::components::auth::use_auth;
use crate::components::uploadthing::{upload_file, UploadResult};
use crate::routes::server::channel::components::chat::ChatContext;

use self::actions::MessageActionButtons;
use self::attachments::AttachmentPreviewList;
use self::input::MessageInputArea;
use self::msg_ref::MsgRefDisplay;

#[derive(Debug, Serialize, Clone)]
pub struct SendMessage {
    #[serde(rename = "channelId")]
    channel: String,
    content: String,
    #[serde(rename = "senderId")]
    sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "referenceId")]
    reference: Option<String>,
}

impl Mutation for SendMessage {
    type Output = String;

    fn name(&self) -> String {
        "messages:createMessage".into()
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct AddAttachment {
    #[serde(rename = "messageId")]
    message: String,
    #[serde(rename = "storageId")]
    storage: String,
}

impl Mutation for AddAttachment {
    type Output = String;

    fn name(&self) -> String {
        "messages:addAttachmentToMessage".into()
    }
}

#[component]
pub fn Sender(
    channel: Signal<Option<Channel>>,
    member: Signal<Option<Member>>,
    sender_ref: NodeRef<Div>,
) -> impl IntoView {
    let send = UseMutation::new::<SendMessage>();
    let auth = use_auth().auth();
    let add_attachment = UseMutation::with_local_fn::<(Vec<File>, String), _, _, _>(
        move |((files, message), client)| {
            let auth = auth.get();
            let mut client_mut = client.to_owned();
            let files = files.to_owned();
            let message = message.to_owned();
            async move {
                if let Some(Ok(Some(auth_data))) = auth {
                    for file in files {
                        let upload_url = GenerateUploadUrl { auth: auth_data.id };
                        let url = upload_url.run(&mut client_mut).await;
                        if let Ok(Some(url)) = url {
                            if let Ok(UploadResult { storage_id }) = upload_file(&file, url).await {
                                let add_attachment = AddAttachment {
                                    message: message.clone(),
                                    storage: storage_id,
                                };
                                let _ = add_attachment.run(&mut client_mut).await;
                            }
                        }
                    }
                }
            }
        },
    );

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
                }
            }
        }
    });

    Effect::watch(
        move || send.value().get(),
        move |message_id_result, _, _| {
            if let Some(Ok(message_id)) = message_id_result {
                message.set(String::default());
                if let Some(div) = content_ref.get() {
                    div.set_inner_text("");
                }
                msg_ref.set(None);
                let current_attachments = attachments.get();
                if !current_attachments.is_empty() {
                    let gloo_files: Vec<File> = current_attachments
                        .into_iter()
                        .map(|file| {
                            File::new_with_options(
                                &file.metadata.name,
                                &*file.chunks,
                                Some(&file.metadata.content_type.to_string()),
                                Some(Utc::now().into()),
                            )
                        })
                        .collect();

                    add_attachment.dispatch_local((gloo_files, message_id.into()));
                    attachments.set(vec![]);
                }
            }
        },
        false,
    );

    view! {
        <div class="w-full absolute bottom-0 bg-transparent flex flex-col z-20 isolate" node_ref=sender_ref>
            <div class="max-h-96 w-full px-5">
                <div class="p-1 border border-input rounded-lg backdrop-blur-xs bg-muted/30">
                    <div class="flex flex-col items-center justify-center shadow-xs bg-background text-base rounded-md gap-2 p-2">
                        <MsgRefDisplay msg_ref=msg_ref on_clear_ref=on_clear_msg_ref/>
                        <AttachmentPreviewList attachments=attachments/>
                        <div class="flex w-full justify-between bg-transparent">
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
                </div>
            </div>
            <div class="absolute inset-0 mb-5 bg-gradient-to-b -z-10 from-transparent to-background"/>
            <div class="bg-background w-full h-5"/>
        </div>
    }
}
