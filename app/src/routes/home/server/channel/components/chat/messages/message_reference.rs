use std::ops::Not;

use common::convex::{ChannelMessage, Member};
use leptos::prelude::*;
use markdown::Markdown;

use crate::routes::server::channel::components::chat::ChatContext;
use icons::{IconCornerUpLeft, IconImage};

#[component]
pub fn ReferencedMessageDisplay(
    referenced_message: ChannelMessage,
    referenced_message_author: Signal<Option<Member>>,
) -> impl IntoView {
    let ChatContext {
        target_message_id, ..
    } = use_context::<ChatContext>().expect("should access to the chat context");

    view! {
        <div class="flex flex-col border-l-2 border-muted-foreground/50 hover:border-muted-foreground/70 hover:bg-accent/70 w-fit px-2 pt-2 bg-accent/50 cursor-pointer transition-colors ease-in-out-quad duration-180"
             on:click=move |_| {
                 target_message_id.set(Some(referenced_message.id.clone()));
             }
        >
            <div class="flex items-center gap-1 text-xs text-muted-foreground">
                <IconCornerUpLeft class="size-3"/>
                <span>
                    {"Replying to "}
                    {move || referenced_message_author.get().map(|m| view!{ <span class="text-foreground">{m.name}</span> })}
                </span>
            </div>
            <div class="text-xs text-muted-foreground flex items-start max-h-8 overflow-hidden line-clamp-2">
                <Markdown source=referenced_message.content />
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
