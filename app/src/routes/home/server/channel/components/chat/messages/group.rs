use std::collections::HashMap;
use std::str::FromStr;

use chrono::{DateTime, Local};
use common::convex::Member;
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

    view! {
        <div class="flex items-start p-3">
            <div>
                {
                    move || {
                        cached_members.get()
                            .and_then(|members_map| {
                                members_map.get(&sender.get_value()).cloned()
                            })
                            .map(|member| view!{
                                <SenderAvatar member=member />
                            })
                    }
                }
            </div>
            <div class="flex flex-col text-sm px-3">
                {
                    move || {
                        cached_members.get()
                            .and_then(|members_map| {
                                members_map.get(&sender.get_value()).cloned()
                            })
                            .map(|member| {
                                let date = get_date(group.creation_time);
                                view!{
                                    <div>
                                        <span>
                                            {member.name}
                                        </span>
                                        <span class="text-muted-foreground text-xs">
                                            " "
                                            {date.map(|date| date.format("%m/%d/%y, %I:%M %p").to_string())}
                                        </span>
                                    </div>
                                }

                        })
                    }
                }
                <For
                    each=move || group.messages.clone()
                    key=|msg_ref| msg_ref.id.clone()
                    children=move |msg_ref| {
                        let markdown = MarkdownParser::new(&msg_ref.content).parse_tree();
                        view! {
                            <Markdown markdown=markdown.into() />
                            {
                                move || {
                                    msg_ref.attachments.iter().map(|att| {
                                        let file_type = FileType::from_str(&att._type);
                                        match file_type {
                                            Ok(FileType::Jpeg) => {
                                                view!{
                                                    <img class="max-w-136 h-auto flex rounded-lg" src=att.url.clone()/>
                                                }.into_any()
                                            },
                                            Ok(FileType::Png) => view!{
                                                    <img class="max-w-136 h-auto flex rounded-lg" src=att.url.clone()/>
                                            }.into_any(),
                                            _ => ().into_any()
                                        }
                                    }).collect_view()
                                }
                            }
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn SenderAvatar(member: Member) -> impl IntoView {
    view! {
        <Avatar class="h-8 w-8 rounded-lg">
            <AvatarImage url=member.image_url/>
            <AvatarFallback class="rounded-lg select-none bg-transparent">
                {member.name.clone()}
            </AvatarFallback>
        </Avatar>
    }
}
