mod group;
use leptos::prelude::*;

use crate::components::ui::divider::Separator;
use crate::components::ui::label::Label;

use self::group::MessageGroup;

use super::MessageDisplayItem;

#[component]
fn DateSeparator(date_string: String) -> impl IntoView {
    view! {
        <Separator class="flex items-center justify-center">
            <Label class="bg-background text-muted-foreground px-1 text-xs">{date_string}</Label>
        </Separator>
    }
}

#[component]
pub fn Messages(messages: Memo<Vec<MessageDisplayItem>>) -> impl IntoView {
    view! {
        <div class="flex min-h-0 flex-1 flex-col gap-2 overflow-auto pt-4 scrollbar-thin scrollbar-track-background">
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
