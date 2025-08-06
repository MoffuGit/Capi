use capi_primitives::menu::{MenuAlign, MenuSide};
use common::convex::{ChannelMessage, Member};
use leptos::prelude::*;

use super::message_content::MessageContent;
use super::message_header::MessageHeader;
use super::message_reactions::MessageReactions;
use super::message_reference::{MessageReferenceButton, ReferencedMessageDisplay};
use crate::components::icons::IconCornerUpLeft;
use crate::components::ui::button::*;
use crate::components::ui::context::*;
use crate::routes::server::channel::components::chat::ChatContext;

#[component]
pub fn MessageItem(
    idx: usize,
    date: f64,
    msg: ChannelMessage,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let msg = StoredValue::new(msg);

    let context: ChatContext = use_context().expect("should return teh chat context");
    let msg_ref = context.msg_reference;
    let cached_members = context.cached_members;

    let msg_value = msg.get_value();

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

    let context_open = RwSignal::new(false);

    view! {
        <ContextMenu open=context_open>
            <ContextMenuTrigger
                class="w-full h-auto transition-colors ease-out-quad duration-180
                    data-[response=true]:bg-purple/10 data-[response=true]:border-l-purple
                    data-[highlight=true]:bg-purple/10 data-[highlight=true]:border-l-purple
                    border-l border-l-transparent data-[context=true]:bg-accent/50 hover:bg-accent/50 px-8 group min-h-9 flex flex-col justify-center relative"
                {..}
                id=msg.get_value().id.clone()
                data-response=move || msg_ref.get().is_some_and(|msg_ref| msg_ref.id == msg.get_value().id).to_string()
                data-highlight=move || context.target_message_id.get().is_some_and(|id| id == msg.get_value().id).to_string()
                data-context=move || context_open.get().to_string()
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
                    { move || {
                        msg.get_value().referenced_message.clone().map(|referenced_msg_data| {
                            view! {
                                <ReferencedMessageDisplay
                                    referenced_message=*referenced_msg_data
                                    referenced_message_author=referenced_message_author
                                />
                            }
                        })
                    }}
                </Show>

                <Show when=move || idx == 0>
                    {
                        move || {
                            member.get()
                                .map(|m| view! { <MessageHeader member=m date=date /> }.into_view())
                        }
                    }
                </Show>

                <MessageContent msg=msg.get_value() />
                <MessageReactions reactions=msg.get_value().reactions />
            </ContextMenuTrigger>
            <ContextMenuContent side=MenuSide::Right align=MenuAlign::Start>
                <Show when=move || context.reactions.get().is_some_and(|reac| !reac.is_empty())>
                    <div class="flex w-full h-auto items-center">
                        <For
                            each=move || context.reactions.get().unwrap_or_default()
                            key=|reaction| reaction.clone()
                            let(reaction)
                        >
                            <Button
                                variant=ButtonVariants::Secondary
                                class="size-4"
                            >
                                {reaction}
                            </Button>
                        </For>
                    </div>
                </Show>
                <ContextMenuItem
                    close_on_click=true
                    {..}
                    on:click=move |_| {
                        context.msg_reference.set(Some(msg.get_value()));
                    }
                >
                    <IconCornerUpLeft />
                    "Reply"
                </ContextMenuItem>
                <ContextMenuItem>
                    "React"
                </ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    }
}
