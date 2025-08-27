mod messages;

use convex_client::leptos::{Query, UseQuery};
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use serde::{Deserialize, Serialize};

use crate::components::auth::use_auth;

use self::messages::Messages;

use super::Header;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrivateMessageDetails {
    pub _id: String,
    pub conversation: String,
    pub sender: String,
    pub content: String,
    pub reference: Option<String>,
    #[serde(rename = "_creationTime")]
    pub _creation_time: i64,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "senderImageUrl")]
    pub sender_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetPrivateMessages {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub auth: i64,
}

impl Query<Vec<PrivateMessageDetails>> for GetPrivateMessages {
    fn name(&self) -> String {
        "privateConversations:getPrivateMessages".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetLastReadMessage {
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    pub auth: i64,
}

impl Query<Option<String>> for GetLastReadMessage {
    fn name(&self) -> String {
        "privateConversations:getLastReadMessage".to_string()
    }
}

#[component]
pub fn Conversation() -> impl IntoView {
    let location = use_location();
    let path = location.pathname;
    let current_conversation = Memo::new(move |_| {
        path.get()
            .split('/')
            .nth(3)
            .map(|conversation| conversation.to_string())
    });
    let auth = use_auth().auth;
    let messages = UseQuery::new(move || {
        let auth = auth.get().and_then(|auth| auth.ok()).flatten()?;
        let conversation = current_conversation.get()?;
        Some(GetPrivateMessages {
            auth: auth.id,
            conversation_id: conversation,
        })
    });
    view! {
        <Header/>
        <div class="flex h-full w-full flex-col relative">
            <Messages messages=messages/>
        </div>
    }
}
