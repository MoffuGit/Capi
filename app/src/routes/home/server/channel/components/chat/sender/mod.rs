mod actions;
mod attachments;
mod input;
mod msg_ref;

use common::convex::{Channel, Member};
use convex_client::leptos::{Mutation, UseMutation};
use leptos::html::Div;
use leptos::prelude::*;
use serde::Serialize;

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
                }
            }
        }
    });

    Effect::watch(
        move || send.value().get(),
        move |message_id_result, _, _| {
            if let Some(Ok(_)) = message_id_result {
                message.set(String::default());
                if let Some(div) = content_ref.get() {
                    div.set_inner_text("");
                }
                msg_ref.set(None);
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
