use capi_ui::avatar::*;
use capi_ui::button::*;
use capi_ui::dropwdown::*;
use chrono::{DateTime, Duration, Local};
use common::convex::{ChannelMessage, Member};
use convex_client::leptos::{Query, UseQuery};
use icons::IconPin;
use leptos::prelude::*;
use markdown::Markdown;
use serde::Serialize;

use crate::routes::server::channel::components::sidebar::card::MemberCard;
use crate::routes::server::channel::Channel;

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct GetPinnedMessages {
    #[serde(rename = "channelId")]
    channel: String,
    #[serde(rename = "memberId")]
    member: String,
}

impl Query<Vec<ChannelMessage>> for GetPinnedMessages {
    fn name(&self) -> String {
        "messages:getPinnedMessages".into()
    }
}

#[component]
pub fn PinnedMessages(
    channel: Signal<Option<Channel>>,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let messages = UseQuery::new(move || {
        let member = member.get()?;
        let channel = channel.get()?;
        Some(GetPinnedMessages {
            channel: channel.id,
            member: member.id,
        })
    });
    view! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button
                    variant=ButtonVariants::Ghost
                    size=ButtonSizes::Icon
                    class="size-7"
                >
                    <IconPin/>
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent side=DropdownMenuSide::Left align=DropdownMenuAlign::Start class="p-0 max-w-full min-w-96">
                <DropdownMenuLabel>
                    "Pinned Messages"
                </DropdownMenuLabel>
                <For
                    each=move || messages.get().and_then(|res| res.ok()).unwrap_or_default()
                    key=|message| message.id.clone()
                    let:message
                >
                    <div class="w-full h-auto flex flex-col gap-1">
                        <div class="w-full p-2 hover:bg-accent/50 flex flex-col text-sm font-light">
                            {
                                move || {
                                    member.get().map(|member| {
                                        view!{
                                            <MessageHeader member=member date=message.creation_time/>
                                        }
                                    })
                                }
                            }
                            <Markdown source=message.content class="prose prose-stone prose-sm dark:prose-invert p-1"/>
                        </div>
                    </div>
                </For>
            </DropdownMenuContent>
        </DropdownMenu>
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

    let member = StoredValue::new(member);

    view! {
        <div class="flex items-center gap-1">
            <DropdownMenu>
                <DropdownMenuTrigger
                    class="cursor-pointer active:scale-[.97]"
                    {..}
                    on:dblclick=move |evt| {
                        evt.stop_propagation();
                    }
                >
                    <Avatar class="flex bg-accent aspect-square size-6 items-center justify-center rounded-md group-data-[state=collapsed]:opacity-0 group-data-[state=expanded]:opacity-100 ease-in-out duration-150 transition-opacity">
                        <AvatarImage url=member.get_value().image_url.clone()/>
                        <AvatarFallback class="rounded-lg select-none bg-transparent">
                            {member.get_value().name.chars().next()}
                        </AvatarFallback>
                    </Avatar>
                </DropdownMenuTrigger>
                <DropdownMenuContent
                    side=DropdownMenuSide::Right
                    align=DropdownMenuAlign::Start
                >
                    <MemberCard member=member.get_value() />
                </DropdownMenuContent>
            </DropdownMenu>
            <div>
                <span>
                    {member.get_value().name}
                </span>
                <span class="text-muted-foreground text-xs ml-1">
                    {formatted_date}
                </span>
            </div>
        </div>
    }
}

pub fn get_date(timestamp_f64: f64) -> Option<DateTime<Local>> {
    DateTime::from_timestamp_millis(timestamp_f64 as i64).map(|date| date.with_timezone(&Local))
}
