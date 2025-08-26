mod group;
mod message_actions;
mod message_attachments;
mod message_content;
mod message_header;
mod message_item;
mod message_reactions;
mod message_reference;
mod utils;

use common::convex::{Channel, ChannelMessage, Member};
use convex_client::leptos::{Mutation, UseMutation};
use leptos::html::Div;
use leptos::prelude::*;
use leptos_dom::warn;
use leptos_use::{
    signal_debounced, signal_debounced_with_options, use_debounce_fn_with_arg,
    use_element_bounding, DebounceOptions, UseElementBoundingReturn,
};

use capi_ui::divider::Separator;
use capi_ui::label::Label;
use serde::Serialize;

use crate::routes::server::channel::components::chat::unread::UnreadMessagesButton;

use self::group::MessageGroup;

use super::MessageDisplayItem;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct UpdateMemberChannelLastRead {
    #[serde(rename = "channelId")]
    pub channel: String,
    #[serde(rename = "messageId")]
    pub message: String,
    #[serde(rename = "memberId")]
    pub member: String,
}

impl Mutation for UpdateMemberChannelLastRead {
    fn name(&self) -> String {
        "unreadMessages:updateMemberChannelLastRead".to_string()
    }

    type Output = ();
}

#[component]
fn DateSeparator(date_string: String) -> impl IntoView {
    view! {
        <div class="py-2">
            <Separator class="flex items-center justify-center">
                <Label class="bg-background text-muted-foreground px-1 text-xs">{date_string}</Label>
            </Separator>
        </div>
    }
}

#[component]
pub fn Messages(
    messages: Memo<Vec<MessageDisplayItem>>,
    sender_ref: NodeRef<Div>,
    channel: Signal<Option<Channel>>,
    member: Signal<Option<Member>>,
    last_read_updated: ReadSignal<Option<Result<Option<ChannelMessage>, String>>>,
) -> impl IntoView {
    let style = RwSignal::new(String::default());
    #[cfg(feature = "hydrate")]
    {
        let UseElementBoundingReturn { height, .. } = use_element_bounding(sender_ref);
        Effect::new(move |_| {
            style.set(format!("--sender-height: {}px", height.get()));
        });
    }
    let (last_read, set_last_read) = signal(None::<ChannelMessage>);
    let debounce_last_read = signal_debounced_with_options::<
        ReadSignal<Option<ChannelMessage>>,
        Option<ChannelMessage>,
    >(
        last_read,
        1000.0,
        DebounceOptions::default().max_wait(500.0),
    );
    let update_last_read = UseMutation::new::<UpdateMemberChannelLastRead>();
    Effect::new(move |_: Option<Option<()>>| {
        if let Some(last_read) = debounce_last_read.get() {
            if let Some(five) = last_read_updated
                .get_untracked()
                .and_then(|s| s.ok())
                .flatten()
            {
                if last_read.creation_time <= five.creation_time {
                    return None;
                }
            } else {
                return None;
            }
            let member = member.get_untracked()?;
            let channel = channel.get_untracked()?;
            update_last_read.dispatch(UpdateMemberChannelLastRead {
                channel: channel.id,
                message: last_read.id,
                member: member.id,
            });
        }
        None
    });
    view! {
        <div style=move || style.get()  class="flex min-h-0 flex-1 flex-col overflow-auto pt-4 scrollbar-thin scrollbar-track-background pb-[var(--sender-height)]">
            {
                move || {
                    messages.get().into_iter().map(|item| {
                        match item {
                            MessageDisplayItem::DateSeparator(date_str) => {
                                view! { <DateSeparator date_string=date_str/> }.into_any()
                            }
                            MessageDisplayItem::MessageGroup(group) => {
                                view! {
                                    <MessageGroup group=group set_last_read=set_last_read />
                                }.into_any()
                            }
                            MessageDisplayItem::UnreadSeparator => {
                                warn!("and we try in here");
                                view! {
                                    <div class="py-2">
                                        <Separator class="flex items-center justify-center">
                                            <Label class="bg-background text-muted-foreground px-1 text-xs">Unread Messages</Label>
                                        </Separator>
                                    </div>
                                }.into_any()
                            }
                        }

                    }).collect_view()
                }
            }
        </div>
    }
}
