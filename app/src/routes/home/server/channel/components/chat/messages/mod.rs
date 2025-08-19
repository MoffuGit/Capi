mod group;
mod message_actions;
mod message_attachments;
mod message_content;
mod message_header;
mod message_item;
mod message_reactions;
mod message_reference;
mod utils;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{use_element_bounding, UseElementBoundingReturn};

use crate::components::ui::divider::Separator;
use crate::components::ui::label::Label;

use self::group::MessageGroup;

use super::MessageDisplayItem;

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
) -> impl IntoView {
    let style = RwSignal::new(String::default());
    #[cfg(feature = "hydrate")]
    {
        let UseElementBoundingReturn { height, .. } = use_element_bounding(sender_ref);
        Effect::new(move |_| {
            style.set(format!("--sender-height: {}px", height.get()));
        });
    }
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
                                    <MessageGroup group=group />
                                }.into_any()
                            }
                        }

                    }).collect_view()
                }
            }
        </div>
    }
}
