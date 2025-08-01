use chrono::{DateTime, Duration, Local};
use common::convex::{ChannelMessage, Member};
use leptos::prelude::*;

use crate::components::icons::IconCornerUpLeft;
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

// #[component]
// pub fn MessageAttachments(attachments: Vec<common::convex::Attachment>) -> impl IntoView {
//     view! {
//         {
//             attachments.into_iter().map(|att| {
//                 let file_type = FileType::from_str(&att._type);
//                 match file_type {
//                     Ok(FileType::Jpeg) => {
//                         view!{
//                             <img class="max-w-136 w-full h-auto flex rounded-lg mb-1" src=att.url.clone()/>
//                         }.into_any()
//                     },
//                     Ok(FileType::Png) => view!{
//                         <img class="max-w-136 w-full h-auto flex rounded-lg" src=att.url.clone()/>
//                     }.into_any(),
//                     _ => ().into_any()
//                 }
//             }).collect_view()
//         }
//     }
// }

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

    view! {
        <div
            data-response=move || if msg_ref.get().is_some_and(|msg_ref| msg_ref.id == msg.get_value().id) { "true" } else { "false" }
            class="w-full h-auto transition-colors ease-out-quad duration-180 data-[response=true]:bg-blue-1/10 data-[response=true]:border-l-blue-1 data-[response=true]:border-l hover:bg-accent/50 px-8 group min-h-9 flex flex-col justify-center relative"
            on:dblclick=move |_| msg_ref.set(Some(msg.get_value()))
        >
            <div class="absolute bg-popover text-popover-foreground flex items-center h-auto z-10 w-auto overflow-x-hidden overflow-y-auto rounded-md border p-1 shadow-md group-hover:opacity-100 opacity-0 top-0 right-4 -translate-y-1/2">
                <MessageReferenceButton msg=msg.get_value() />
            </div>
            <Show when=move || idx == 0>
                {
                    move || {
                        member.get()
                            .map(|m| view! { <MessageHeader member=m date=date /> }.into_view())
                    }
                }
            </Show>

            <Markdown markdown=markdown.into() />
            // <MessageAttachments attachments=msg.get_value().attachments />
        </div>
    }
}
