mod dropzone;
mod messages;
mod sender;
mod unread;

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local, NaiveDate};
use common::files::ClientFile;
use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;

use common::convex::{Channel, ChannelMessage, Member};
use leptos::context::Provider;
use serde::Serialize;

use self::messages::Messages;
use self::sender::Sender;
use self::unread::UnreadMessagesButton;

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct GetMemberEmojis {
    #[serde(rename = "memberId")]
    member: String,
}

impl Query<Vec<String>> for GetMemberEmojis {
    fn name(&self) -> String {
        "reaction:getMemberEmojis".into()
    }
}

#[derive(Debug, Clone)]
pub struct ChatContext {
    pub member: Signal<Option<Member>>,
    pub msg_reference: RwSignal<Option<ChannelMessage>>,
    pub attachments: RwSignal<Vec<ClientFile>>,
    pub cached_members: Memo<Option<HashMap<String, Member>>>,
    pub target_message_id: RwSignal<Option<String>>,
    pub reactions: Signal<Option<Vec<String>>>,
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
    UnreadSeparator,
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

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetLastReadMessageId {
    #[serde(rename(serialize = "memberId"))]
    pub member_id: String,
    #[serde(rename(serialize = "channelId"))]
    pub channel_id: String,
}

impl Query<Option<ChannelMessage>> for GetLastReadMessageId {
    fn name(&self) -> String {
        "unreadMessages:getLastReadMessageId".to_string()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetUnreadMessagesCountInChannel {
    #[serde(rename(serialize = "memberId"))]
    pub member_id: String,
    #[serde(rename(serialize = "channelId"))]
    pub channel_id: String,
}

impl Query<f64> for GetUnreadMessagesCountInChannel {
    fn name(&self) -> String {
        "unreadMessages:getUnreadMessagesCountInChannel".to_string()
    }
}

#[component]
pub fn Chat(channel: Signal<Option<Channel>>, member: Signal<Option<Member>>) -> impl IntoView {
    let messages = UseQuery::new(move || {
        let member = member.get()?;
        let channel = channel.get()?;
        Some(GetMessagesInChannel {
            channel: channel.id,
            member: member.id,
        })
    });
    let last_read_message_id = UseQuery::new(move || {
        let member = member.get()?;
        let channel = channel.get()?;
        Some(GetLastReadMessageId {
            member_id: member.id,
            channel_id: channel.id,
        })
    });

    let unread_count = UseQuery::new(move || {
        let member = member.get()?;
        let channel = channel.get()?;
        Some(GetUnreadMessagesCountInChannel {
            member_id: member.id,
            channel_id: channel.id,
        })
    });

    let display_items_memo = Memo::new(move |_| {
        let msgs = messages.get().and_then(|res| res.ok()).unwrap_or_default();
        let mut items: Vec<MessageDisplayItem> = Vec::new();
        let mut last_processed_date: Option<NaiveDate> = None;
        let mut current_grouped_message: Option<GroupedMessage> = None;
        let mut has_found_unread_separator = false;
        let mut has_found_unread_message = false;
        let last_read_id = last_read_message_id
            .get()
            .and_then(|last| last.ok())
            .flatten();

        for message in msgs {
            let current_msg_date = get_naive_date_from_convex_timestamp(message.creation_time);
            let current_author_id = message.sender.clone();

            let is_new_day = match (current_msg_date, last_processed_date) {
                (Some(current_d), Some(last_d)) => current_d != last_d,
                (Some(_), None) => true,
                _ => false,
            };

            if is_new_day {
                if let Some(group) = current_grouped_message.take() {
                    items.push(MessageDisplayItem::MessageGroup(group));
                }

                if let Some(date) = current_msg_date {
                    items.push(MessageDisplayItem::DateSeparator(
                        date.format("%B %d, %Y").to_string(),
                    ));
                }
                last_processed_date = current_msg_date;
            }

            let needs_new_group = match &current_grouped_message {
                Some(group) => {
                    let group_creation_date =
                        get_naive_date_from_convex_timestamp(group.creation_time);
                    current_author_id != group.author_id || current_msg_date != group_creation_date
                }
                None => true,
            };
            let msg_creation_time = message.creation_time;

            if needs_new_group {
                if let Some(group) = current_grouped_message.take() {
                    items.push(MessageDisplayItem::MessageGroup(group));
                }
                current_grouped_message = Some(GroupedMessage {
                    author_id: current_author_id,
                    creation_time: message.creation_time,
                    messages: vec![message],
                });
            } else if let Some(group) = current_grouped_message.as_mut() {
                group.messages.push(message);
            }

            if !has_found_unread_message {
                if let Some(ref last_read_id_str) = last_read_id {
                    if msg_creation_time > last_read_id_str.creation_time {
                        has_found_unread_message = true;
                    }
                }

                if has_found_unread_message && !has_found_unread_separator {
                    items.push(MessageDisplayItem::UnreadSeparator);
                    has_found_unread_separator = true;
                }
            }
        }

        if let Some(group) = current_grouped_message.take() {
            items.push(MessageDisplayItem::MessageGroup(group));
        }

        items
    });

    let sender_ids_map = Memo::new(move |_| {
        let msgs = messages.get().and_then(|res| res.ok()).unwrap_or_default();
        let mut unique_senders = HashSet::new();
        for message in msgs {
            unique_senders.insert(message.sender);
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

    let sender_ref = NodeRef::new();

    let target_message_id = RwSignal::new(None);

    Effect::new(move |prev: Option<Option<TimeoutHandle>>| {
        if target_message_id.get().is_some() {
            if let Some(Some(prev)) = prev {
                prev.clear();
            }
            set_timeout_with_handle(
                move || {
                    target_message_id.set(None);
                },
                std::time::Duration::from_secs(3),
            )
            .ok()
        } else {
            None
        }
    });
    let member_reactions = UseQuery::new(move || {
        member
            .get()
            .map(|member| GetMemberEmojis { member: member.id })
    });

    let reactions = Signal::derive(move || member_reactions.get().and_then(|res| res.ok()));

    let unread_count_signal =
        Signal::derive(move || unread_count.get().and_then(|res| res.ok()).unwrap_or(0.0));

    let last_read_message_signal = Signal::derive(move || {
        last_read_message_id
            .get()
            .and_then(|res| res.ok())
            .flatten()
    });

    let scroll_to_message_id = Callback::new(move |message_id: String| {
        target_message_id.set(Some(message_id));
    });

    view! {
        <Provider value=ChatContext {
            member,
            msg_reference: RwSignal::new(None),
            attachments: RwSignal::new(vec![]),
            cached_members,
            target_message_id,
            reactions
        }>
            <div class="flex h-full w-full flex-col relative">
                <UnreadMessagesButton
                    unread_count=unread_count_signal
                    last_read_message=last_read_message_signal
                    scroll_to_message=scroll_to_message_id
                />
                <Messages messages=display_items_memo sender_ref=sender_ref member=member channel=channel last_read_updated=last_read_message_id/>
                <Sender channel=channel member=member sender_ref=sender_ref/>
            </div>
        </Provider>
    }
}
