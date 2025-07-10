mod dropzone;
mod messages;
mod sender;

use leptos::prelude::*;

use common::convex::{Channel, ChannelMessage, Member};
use leptos::context::Provider;

use self::messages::Messages;
use self::sender::Sender;

#[derive(Debug, Clone, Default)]
pub struct ChatContext {
    pub msg_reference: RwSignal<Option<ChannelMessage>>,
    pub attachments: RwSignal<Vec<String>>,
}

#[component]
pub fn Chat(
    channel: RwSignal<Option<Channel>>,
    member: RwSignal<Option<Option<Member>>>,
) -> impl IntoView {
    let chat_context = ChatContext::default();
    view! {
        <Provider value=chat_context>
            // <ChatDropZone/>
            <div class="flex h-full w-full flex-col">
                <Messages channel=channel/>
                <Sender channel=channel member=member/>
            </div>
        </Provider>
    }
}
