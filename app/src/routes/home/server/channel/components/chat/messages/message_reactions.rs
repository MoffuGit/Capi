use common::convex::{ChannelMessage, Member};
use convex_client::leptos::{Mutation, UseMutation};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::ui::button::*;

#[component]
pub fn MessageReactions(
    msg: StoredValue<ChannelMessage>,
    member: Signal<Option<Member>>,
) -> impl IntoView {
    let add_reaction = UseMutation::new::<AddReaction>();
    let remove_reaction = UseMutation::new::<RemoveReaction>();

    view! {
        <div class="flex w-auto h-auto items-center gap-1 pb-1">
            <For
                each=move || msg.get_value().reactions
                key=|reaction| reaction.id.clone()
                children=move |reaction| {
                    let emoji = StoredValue::new(reaction.emoji);
                    view!{
                        <Button
                            variant=Signal::derive(move || {
                                if reaction.has_reacted {
                                    ButtonVariants::Secondary
                                } else {
                                    ButtonVariants::Ghost
                                }
                            })
                            size=ButtonSizes::Sm
                            on:click=move |_| {
                                if let Some(member) = member.get() {
                                    if reaction.has_reacted {
                                        remove_reaction.dispatch(RemoveReaction {
                                            message: msg.get_value().id,
                                            member: member.id,
                                            emoji: emoji.get_value(),
                                        });
                                    } else {
                                        add_reaction.dispatch(AddReaction {
                                            message: msg.get_value().id,
                                            member: member.id,
                                            emoji: emoji.get_value()
                                        });
                                    }

                                }
                            }
                        >
                            <span>
                                {emoji.get_value()}
                            </span>
                            <span>
                                {reaction.count as u64}
                            </span>
                        </Button>

                    }
                }
            />
        </div>
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct AddReaction {
    #[serde(rename = "messageId")]
    pub message: String,
    #[serde(rename = "memberId")]
    pub member: String,
    pub emoji: String,
}

impl Mutation for AddReaction {
    type Output = ();

    fn name(&self) -> String {
        "reaction:addReaction".into()
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct RemoveReaction {
    #[serde(rename = "messageId")]
    message: String,
    #[serde(rename = "memberId")]
    member: String,
    emoji: String,
}

impl Mutation for RemoveReaction {
    type Output = ();

    fn name(&self) -> String {
        "reaction:removeReaction".into()
    }
}
