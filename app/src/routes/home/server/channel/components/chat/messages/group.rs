use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Duration, Local};
use common::convex::{ChannelMessage, Member};
use leptos::prelude::*;
use uploadthing::FileType;

use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::markdown::{Markdown, MarkdownParser};

use super::GroupedMessage;

pub fn get_date(timestamp_f64: f64) -> Option<DateTime<Local>> {
    DateTime::from_timestamp_millis(timestamp_f64 as i64).map(|date| date.with_timezone(&Local))
}

#[component]
pub fn MessageGroup(
    group: GroupedMessage,
    cached_members: Memo<Option<HashMap<String, Member>>>,
) -> impl IntoView {
    let sender = StoredValue::new(group.author_id);
    let member = Signal::derive(move || {
        cached_members
            .get()
            .and_then(|members_map| members_map.get(&sender.get_value()).cloned())
    });

    view! {
        <div class="flex items-start relative py-2">
            <div class="flex flex-col text-sm w-full">
                <For
                    each=move || group.messages.clone().into_iter().enumerate()
                    key=|(_, msg)| msg.id.clone()
                    children=move |(idx, msg_ref)| {
                        view!{
                            <MessageItem idx=idx author=sender.get_value() date=group.creation_time member=member msg=msg_ref/>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn MessageItem(
    idx: usize,
    author: String,
    date: f64,
    msg: ChannelMessage,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let markdown = MarkdownParser::new(&msg.content).parse_tree();
    view! {
        <div class="w-full h-auto hover:bg-accent/50 px-18 relative">
            {
                move || {
                    if idx == 0 {
                        member.get()
                            .map(|member| {
                                let date = get_date(date);
                                view!{
                                        <span class="font-medium">
                                            {member.name.clone()}
                                        </span>
                                        <span class="text-muted-foreground text-xs ml-1">
                                            {date.map(|date| {
                                                let today = Local::now().date_naive();
                                                let yesterday = (Local::now() - Duration::days(1)).date_naive();
                                                let message_date = date.date_naive();

                                                if message_date == today {
                                                    format!("today at {}", date.format("%I:%M %p"))
                                                } else if message_date == yesterday {
                                                    format!("yesterday at {}", date.format("%I:%M %p"))
                                                } else {
                                                    date.format("%m/%d/%y, %I:%M %p").to_string()
                                                }
                                            })}
                                        </span>
                                    <SenderAvatar member=member />
                                }
                            })

                    } else {
                        None
                    }
                }
            }

            <Markdown markdown=markdown.into() />
            {
                move || {
                    msg.attachments.iter().map(|att| {
                        let file_type = FileType::from_str(&att._type);
                        match file_type {
                            Ok(FileType::Jpeg) => {
                                view!{
                                    <img class="max-w-136 w-full h-auto flex rounded-lg mb-1" src=att.url.clone()/>
                                }.into_any()
                            },
                            Ok(FileType::Png) => view!{
                                    <img class="max-w-136 w-full h-auto flex rounded-lg" src=att.url.clone()/>
                            }.into_any(),
                            _ => ().into_any()
                        }
                    }).collect_view()
                }
            }
        </div>
    }
}

#[component]
pub fn SenderAvatar(member: Member) -> impl IntoView {
    view! {
        <Avatar class="h-8 w-8 rounded-lg absolute top-1 left-6">
            <AvatarImage url=member.image_url/>
            <AvatarFallback class="rounded-lg select-none bg-transparent">
                {member.name.clone()}
            </AvatarFallback>
        </Avatar>
    }
}
