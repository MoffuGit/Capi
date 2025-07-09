use common::convex::{Channel, ChannelMessage};
use leptos::prelude::*;

use api::convex::Query;
use serde_json::json;

use crate::hooks::sycn::SyncSignal;

#[component]
pub fn Messages(channel: RwSignal<Option<Channel>>) -> impl IntoView {
    let messages: SyncSignal<Vec<ChannelMessage>> = SyncSignal::new(Memo::new(move |_| {
        channel.get().map(|channel| Query {
            name: "messages:getMessagesInChannel".to_string(),
            args: json!({
                "channelId": channel.id
            }),
        })
    }));

    view! {
        <Show when=move || messages.signal.get().is_some()>
            <For
                each=move || messages.signal.get().unwrap()
                key=|message| message.id.clone()
                children=move |message| {
                    view!{
                        <div>
                            {message.content}
                        </div>
                    }
                }
            />
        </Show>
    }
}
