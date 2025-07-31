mod dropzone;
mod messages;
mod sender;

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local, NaiveDate};
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;

use common::convex::{Channel, ChannelMessage, Member};
use leptos::context::Provider;
use serde::Serialize;
use uploadthing::UploadthingFile;

use self::messages::Messages;
use self::sender::Sender;

#[derive(Debug, Clone)]
pub struct ChatContext {
    pub msg_reference: RwSignal<Option<ChannelMessage>>,
    pub attachments: RwSignal<Vec<UploadthingFile>>,
    pub cached_members: Memo<Option<HashMap<String, Member>>>,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetMessagesInChannel {
    #[serde(rename(serialize = "channelId"))]
    channel: String,
    #[serde(rename(serialize = "memberId"))]
    member: String,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetMembersById {
    #[serde(rename(serialize = "memberIds"))]
    members: Vec<String>,
}

impl Query<Vec<Member>> for GetMembersById {
    fn name(&self) -> String {
        "member:getMembersByIds".to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MessageDisplayItem {
    DateSeparator(String),
    MessageGroup(GroupedMessage),
}

impl Query<Vec<ChannelMessage>> for GetMessagesInChannel {
    fn name(&self) -> String {
        "messages:getMessagesInChannel".to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct GroupedMessage {
    author_id: String,
    creation_time: f64,
    messages: Vec<ChannelMessage>,
}

pub fn get_naive_date_from_convex_timestamp(timestamp_f64: f64) -> Option<NaiveDate> {
    let dt = DateTime::from_timestamp_millis(timestamp_f64 as i64)?;
    Some(dt.with_timezone(&Local).date_naive())
}

#[component]
pub fn Chat(channel: Signal<Option<Channel>>, member: Signal<Option<Member>>) -> impl IntoView {
    let messages = UseQuery::new(move || {
        member.get().and_then(|member| {
            channel.get().map(|channel| GetMessagesInChannel {
                channel: channel.id,
                member: member.id,
            })
        })
    });
    let grouped_messages_data = Memo::new(move |_| {
        let mut grouped_messages: Vec<GroupedMessage> = Vec::new();
        let msgs = messages.get().and_then(|res| res.ok()).unwrap_or_default();

        for message in msgs.into_iter() {
            let current_msg_date = get_naive_date_from_convex_timestamp(message.creation_time);
            let current_author_id = message.sender.clone();

            let mut start_new_group = false;

            if grouped_messages.is_empty() {
                start_new_group = true;
            } else {
                let last_group = grouped_messages.last().unwrap();
                let last_group_author_id = &last_group.author_id;
                let last_group_first_msg_date =
                    get_naive_date_from_convex_timestamp(last_group.creation_time);

                if last_group_author_id != &current_author_id
                    || current_msg_date != last_group_first_msg_date
                {
                    start_new_group = true;
                }
            }

            if start_new_group {
                grouped_messages.push(GroupedMessage {
                    author_id: current_author_id,
                    creation_time: message.creation_time,
                    messages: vec![message],
                });
            } else if let Some(last_group) = grouped_messages.last_mut() {
                last_group.messages.push(message);
            }
        }
        grouped_messages
    });

    let display_items_memo = Memo::new(move |_| {
        let mut items: Vec<MessageDisplayItem> = Vec::new();
        let grouped_msgs = grouped_messages_data.get();
        let mut last_processed_date: Option<NaiveDate> = None;

        for group in grouped_msgs.into_iter() {
            let current_group_date = get_naive_date_from_convex_timestamp(group.creation_time);

            let needs_separator = match (current_group_date, last_processed_date) {
                (Some(current_date), Some(last_date_val)) => current_date != last_date_val,
                (Some(_), None) => true,
                _ => false,
            };

            if needs_separator {
                if let Some(date) = current_group_date {
                    items.push(MessageDisplayItem::DateSeparator(
                        date.format("%B %d, %Y").to_string(),
                    ));
                }
            }

            items.push(MessageDisplayItem::MessageGroup(group));

            last_processed_date = current_group_date;
        }
        items
    });

    let sender_ids_map = Memo::new(move |_| {
        let grouped_msgs = grouped_messages_data.get();
        let mut unique_senders = HashSet::new();
        for group in grouped_msgs {
            unique_senders.insert(group.author_id.clone());
        }
        unique_senders
    });

    let members_data = UseQuery::new(move || {
        Some(GetMembersById {
            members: sender_ids_map.get().iter().cloned().collect(),
        })
    });

    let cached_members: Memo<Option<HashMap<String, Member>>> = Memo::new(move |_| {
        members_data.get().and_then(|res| res.ok()).map(|members| {
            members
                .into_iter()
                .map(|member| (member.id.clone(), member))
                .collect()
        })
    });

    view! {
        <Provider value=ChatContext {
            msg_reference: Default::default(),
            attachments: Default::default(),
            cached_members
        }>
            <div class="flex h-full w-full flex-col relative">
                <Messages messages=display_items_memo/>
                <Sender channel=channel member=member/>
            </div>
        </Provider>
    }
}
