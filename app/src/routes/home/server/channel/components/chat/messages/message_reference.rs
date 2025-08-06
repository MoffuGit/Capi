use std::ops::Not;

use common::convex::{ChannelMessage, Member};
use leptos::prelude::*;

use crate::components::icons::{IconCornerUpLeft, IconImage};
use crate::components::ui::button::{Button, ButtonSizes, ButtonVariants};
use crate::components::ui::markdown::{Markdown, MarkdownParser};
use crate::routes::server::channel::components::chat::ChatContext;

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
pub fn ReferencedMessageDisplay(
    referenced_message: ChannelMessage,
    referenced_message_author: Signal<Option<Member>>,
) -> impl IntoView {
    let ChatContext {
        target_message_id, ..
    } = use_context::<ChatContext>().expect("should access to the chat context");

    let referenced_message_content_markdown =
        MarkdownParser::new(&referenced_message.content).parse_tree();

    view! {
        <div class="flex flex-col border-l-2 border-muted-foreground/50 pl-2 mb-1 mt-1 bg-accent/50 cursor-pointer"
             on:click=move |_| {
                 target_message_id.set(Some(referenced_message.id.clone()));
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
                <Markdown markdown=referenced_message_content_markdown.into() />
                {
                    referenced_message.attachments.is_empty().not().then(|| {
                        view!{
                            <IconImage class="size-4 ml-1" />
                        }
                    })
                }
            </div>
        </div>
    }
}
