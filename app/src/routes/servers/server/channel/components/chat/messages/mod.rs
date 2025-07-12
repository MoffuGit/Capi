mod group;
use chrono::{DateTime, Local, NaiveDate};
use common::convex::{Channel, ChannelMessage, Member};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

use api::convex::Query;
use serde_json::json;

use crate::components::ui::divider::Separator;
use crate::components::ui::label::Label;
use crate::hooks::sycn::SyncSignal;

use self::group::MessageGroup;

pub fn get_naive_date_from_convex_timestamp(timestamp_f64: f64) -> Option<NaiveDate> {
    let dt = DateTime::from_timestamp_millis(timestamp_f64 as i64)?;
    Some(dt.with_timezone(&Local).date_naive())
}

#[derive(Debug, PartialEq, Clone)]
struct GroupedMessage {
    author_id: String,
    creation_time: f64,
    messages: Vec<ChannelMessage>,
}

#[derive(Debug, PartialEq, Clone)]
enum MessageDisplayItem {
    DateSeparator(String),
    MessageGroup(GroupedMessage),
}

#[component]
fn DateSeparator(date_string: String) -> impl IntoView {
    view! {
        <Separator class="flex items-center justify-center my-3">
            <Label class="bg-background text-muted-foreground px-1 text-xs">{date_string}</Label>
        </Separator>
    }
}

#[component]
pub fn Messages(channel: RwSignal<Option<Channel>>) -> impl IntoView {
    let messages: SyncSignal<Vec<ChannelMessage>> = SyncSignal::new(Memo::new(move |_| {
        channel.get().map(|channel| Query {
            name: "messages:getMessagesInChannel".to_string(),
            args: json!({
                "channelId": channel.id
            }),
        })
    }));

    let grouped_messages_data = Memo::new(move |_| {
        let mut grouped_messages: Vec<GroupedMessage> = Vec::new();
        let msgs = messages.signal.get().unwrap_or_default();

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

    let members_data: SyncSignal<Vec<Member>> = SyncSignal::new(Memo::new(move |_| {
        let ids: Vec<String> = sender_ids_map.get().iter().cloned().collect();
        Some(Query {
            name: "member:getMembersByIds".to_string(),
            args: json!({
                "memberIds": ids
            }),
        })
    }));

    let cached_members: Memo<Option<HashMap<String, Member>>> = Memo::new(move |_| {
        members_data.signal.get().map(|members| {
            members
                .into_iter()
                .map(|member| (member.id.clone(), member))
                .collect()
        })
    });

    view! {
        <div class="flex min-h-0 flex-1 flex-col gap-2 overflow-auto">
                {
                    move || {
                        display_items_memo.get().into_iter().map(|item| {
                            match item {
                                MessageDisplayItem::DateSeparator(date_str) => {
                                    view! { <DateSeparator date_string=date_str/> }.into_any()
                                }
                                MessageDisplayItem::MessageGroup(group) => {
                                    view! {
                                        <MessageGroup group=group cached_members=cached_members />
                                    }.into_any()
                                }
                            }

                        }).collect_view()
                    }
                }
        </div>
    }
}
