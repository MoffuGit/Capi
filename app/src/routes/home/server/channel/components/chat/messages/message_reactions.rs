use common::convex::Reaction;
use convex_client::leptos::{Mutation, Query};
use leptos::prelude::*;
use serde::Serialize;

use crate::components::ui::button::*;

//NOTE: this will have at the end a button for adding more reactions,
//it need to check if the user already have reacted with an emoji, if true, on click will remove,
//if not then will react
#[component]
pub fn MessageReactions(reactions: Vec<Reaction>) -> impl IntoView {
    view! {
        <div class="flex w-auto h-auto items-center gap-1">
            <For
                each=move || reactions.clone()
                key=|reaction| reaction.id.clone()
                let(
                    reaction
                )
            >
                <Button
                    variant=ButtonVariants::Secondary
                    class="size-4"
                >
                    {reaction.emoji}
                </Button>
            </For>
        </div>
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct AddReaction {
    #[serde(rename = "messageId")]
    message: String,
    #[serde(rename = "memberId")]
    member: String,
    emoji: String,
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
        "reaction:RemoveReaction".into()
    }
}
