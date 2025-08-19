use leptos::prelude::*;

use crate::routes::server::channel::components::chat::messages::message_item::MessageItem;
use crate::routes::server::channel::components::chat::{ChatContext, GroupedMessage};

#[component]
pub fn MessageGroup(group: GroupedMessage) -> impl IntoView {
    let context: ChatContext = use_context().expect("should return teh chat context");
    let cached_members = context.cached_members;
    let sender = StoredValue::new(group.author_id);
    let sender = Signal::derive(move || {
        cached_members
            .get()
            .and_then(|members_map| members_map.get(&sender.get_value()).cloned())
    });

    view! {
        <div class="flex items-start relative">
            <div class="flex flex-col text-sm font-light w-full">
                <For
                    each=move || group.messages.clone().into_iter().enumerate()
                    key=|(_, msg)| msg.id.clone()
                    children=move |(idx, msg_ref)| {
                        view!{
                            <MessageItem idx=idx date=group.creation_time sender=sender msg=msg_ref/>
                        }
                    }
                />
            </div>
        </div>
    }
}
