use std::ops::Not;

use chrono::{DateTime, Duration, Local};
use common::convex::{ChannelMessage, FileType, Member};
use leptos::prelude::*;

use crate::components::icons::{IconCornerUpLeft, IconImage};
use crate::components::ui::avatar::{Avatar, AvatarFallback, AvatarImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::markdown::{Markdown, MarkdownParser};
use crate::routes::server::channel::components::chat::{ChatContext, GroupedMessage};

pub fn get_date(timestamp_f64: f64) -> Option<DateTime<Local>> {
    DateTime::from_timestamp_millis(timestamp_f64 as i64).map(|date| date.with_timezone(&Local))
}

#[component]
pub fn MessageGroup(group: GroupedMessage) -> impl IntoView {
    let context: ChatContext = use_context().expect("should return teh chat context");
    let cached_members = context.cached_members;
    let sender = StoredValue::new(group.author_id);
    let member = Signal::derive(move || {
        cached_members
            .get()
            .and_then(|members_map| members_map.get(&sender.get_value()).cloned())
    });

    view! {
        <div class="flex items-start relative py-2">
            <div class="flex flex-col text-sm font-light w-full">
                <For
                    each=move || group.messages.clone().into_iter().enumerate()
                    key=|(_, msg)| msg.id.clone()
                    children=move |(idx, msg_ref)| {
                        view!{
                            <MessageItem idx=idx date=group.creation_time member=member msg=msg_ref/>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
pub fn MessageReferenceButton(msg: ChannelMessage) -> impl IntoView {
    let ChatContext { msg_reference, .. } =
        use_context::<ChatContext>().expect("should access to the chat context");
    view! {
        <Button
            variant=ButtonVariants::Ghost
            size=ButtonSizes::Icon class="size-6"
            on:click=move |_| {
                msg_reference.set(Some(msg.clone()));
            }
        >
            <IconCornerUpLeft />
        </Button>
    }
}

#[component]
pub fn MessageHeader(member: Member, date: f64) -> impl IntoView {
    let formatted_date = Signal::derive(move || {
        get_date(date).map(|date| {
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
        })
    });

    let name = StoredValue::new(member.name.clone());

    view! {
        <div class="pt-2 flex items-center gap-1">
            <Avatar class="flex bg-accent aspect-square size-6 items-center justify-center rounded-md group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                <AvatarImage url=member.image_url.clone()/>
                <AvatarFallback class="rounded-lg select-none bg-transparent">
                    {name.get_value().chars().next()}
                </AvatarFallback>
            </Avatar>
            <div>
                <span class="font-medium">
                    {name.get_value()}
                </span>
                <span class="text-muted-foreground text-xs ml-1">
                    {formatted_date}
                </span>
            </div>
        </div>
    }
}

#[component]
pub fn MessageAttachments(attachments: Vec<common::convex::Attachment>) -> impl IntoView {
    view! {
        {
            attachments.into_iter().map(|att| {
                att.metadata.map(|data| {
                    match data.content_type {
                        FileType::Jpeg | FileType::Png => {
                            view!{
                                <img class="max-w-136 w-full h-auto flex rounded-lg mb-1" src=att.url.clone()/>
                            }.into_any()
                        },
                        _ => ().into_any()
                    }
                })
            }).collect_view()
        }
    }
}

#[component]
pub fn MessageItem(
    idx: usize,
    date: f64,
    msg: ChannelMessage,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let markdown = MarkdownParser::new(&msg.content).parse_tree();
    let msg = StoredValue::new(msg);

    let context: ChatContext = use_context().expect("should return teh chat context");
    let msg_ref = context.msg_reference;
    let cached_members = context.cached_members;
    let target_message_id = context.target_message_id; // Assume ChatContext has this field

    let msg_value = msg.get_value();

    let referenced_message_content_markdown = msg
        .get_value()
        .referenced_message
        .as_ref()
        .map(|m| MarkdownParser::new(&m.content).parse_tree());

    let referenced_message_author_id_option = msg_value
        .referenced_message
        .as_ref()
        .map(|m| m.sender.clone());

    let referenced_message_author = Signal::derive(move || {
        if let Some(author_id) = referenced_message_author_id_option.clone() {
            cached_members
                .get()
                .and_then(|members_map| members_map.get(&author_id).cloned())
        } else {
            None
        }
    });

    view! {
        <div
            id=msg.get_value().id.clone() // Add ID for scrolling
            data-response=move || if msg_ref.get().is_some_and(|msg_ref| msg_ref.id == msg.get_value().id) { "true" } else { "false" }
            data-highlight=move || if target_message_id.get().is_some_and(|id| id == msg.get_value().id) { "true" } else { "false" }
            class="w-full h-auto transition-colors ease-out-quad duration-180
                   dark:data-[response=true]:bg-purple/10 dark:data-[response=true]:border-l-purple
                   data-[response=true]:bg-red/10 data-[response=true]:border-l-red
                   dark:data-[highlight=true]:bg-purple/10 dark:data-[highlight=true]:border-l-purple
                   data-[highlight=true]:bg-red/10 data-[highlight=true]:border-l-red
                   border-l border-l-transparent hover:bg-accent/50 px-8 group min-h-9 flex flex-col justify-center relative"
            on:dblclick=move |_| {
                msg_ref.update(|current| {
                    let msg = Some(msg.get_value());
                    if current == &msg {
                        *current = None;
                    } else {
                        *current = msg;
                    }
                });
            }
        >
            <div class="absolute bg-popover text-popover-foreground flex items-center h-auto z-10 w-auto overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md group-hover:opacity-100 opacity-0 top-0 right-4 -translate-y-1/2">
                <MessageReferenceButton msg=msg.get_value() />
            </div>

            <Show when=move || msg.get_value().referenced_message.is_some()>
                <div class="flex flex-col border-l-2 border-muted-foreground/50 pl-2 mb-1 mt-1 bg-accent/50 cursor-pointer"
                     on:click=move |_| {
                         if let Some(referenced_msg) = msg.get_value().referenced_message.as_ref().map(|m| m.id.clone()) {
                             target_message_id.set(Some(referenced_msg));
                         }
                     }
                >
                    <div class="flex items-center gap-1 text-xs text-muted-foreground">
                        <IconCornerUpLeft class="size-3"/>
                        <span>
                            {"Replying to "}
                            {move || referenced_message_author.get().map(|m| view!{ <span class="font-semibold text-foreground">{m.name}</span> })}
                        </span>
                    </div>
                    <div class="text-xs text-muted-foreground flex items-center max-h-8 overflow-hidden line-clamp-2">
                        {referenced_message_content_markdown.clone().map(|md| view! {
                            <Markdown markdown=md.into() />
                        })}
                        {
                            msg.get_value().referenced_message.and_then(|msg| {
                                msg.attachments.is_empty().not().then(|| {
                                    view!{
                                        <IconImage class="size-4 ml-1" />
                                    }
                                })
                            })
                        }
                    </div>
                </div>
            </Show>

            <Show when=move || idx == 0>
                {
                    move || {
                        member.get()
                            .map(|m| view! { <MessageHeader member=m date=date /> }.into_view())
                    }
                }
            </Show>

            <Markdown markdown=markdown.into() />
            <MessageAttachments attachments=msg.get_value().attachments />
        </div>
    }
}
