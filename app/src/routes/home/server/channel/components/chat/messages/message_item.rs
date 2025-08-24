use common::convex::{ChannelMessage, Member};
use convex_client::leptos::UseMutation;
use emojis::Emoji;
use leptos::prelude::*;

use super::message_content::MessageContent;
use super::message_header::MessageHeader;
use super::message_reactions::MessageReactions;
use super::message_reference::ReferencedMessageDisplay;
use crate::components::emojis::EmojiSelector;
use crate::routes::server::channel::components::chat::messages::message_actions::MessageActions;
use crate::routes::server::channel::components::chat::messages::message_reactions::AddReaction;
use crate::routes::server::channel::components::chat::ChatContext;
use capi_ui::context::*;
use icons::IconCornerUpLeft;

#[component]
pub fn MessageItem(
    idx: usize,
    date: f64,
    msg: ChannelMessage,
    sender: Signal<Option<Member>>,
) -> impl IntoView {
    let msg = StoredValue::new(msg);

    let context: ChatContext = use_context().expect("should return teh chat context");
    let member = context.member;
    let msg_ref = context.msg_reference;
    let cached_members = context.cached_members;

    let msg_value = msg.get_value();

    let referenced_message_author_id_option = msg_value
        .referenced_message
        .as_ref()
        .map(|m| m.sender.clone());

    let add_reaction = UseMutation::new::<AddReaction>();
    let on_select_emoji = Callback::new(move |emoji: &'static Emoji| {
        if let Some(member) = member.get() {
            add_reaction.dispatch(AddReaction {
                message: msg.get_value().id,
                member: member.id,
                emoji: emoji.to_string(),
            });
        }
    });

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
                class="w-full h-auto transition-colors ease-in-out-quad duration-180 gap-2 py-1
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
                <MessageActions msg=msg member=member/>

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
                            sender.get()
                                .map(|m| view! { <MessageHeader member=m date=date /> }.into_any())
                        }
                    }
                </Show>

                <MessageContent msg=msg.get_value() />
                <MessageReactions msg=msg member=member />
            </ContextMenuTrigger>
            <ContextMenuContent side=ContextMenuSide::Right align=ContextMenuAlign::Start>
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
                <ContextSubMenu>
                    <ContextSubTrigger>
                        "Reaction"
                    </ContextSubTrigger>
                    <ContextSubContent side=ContextMenuSide::Right align=ContextMenuAlign::Center>
                        <EmojiSelector history=context.reactions class="p-1" on_select_emoji=on_select_emoji/>
                    </ContextSubContent>
                </ContextSubMenu>
            </ContextMenuContent>
        </ContextMenu>
    }
}
