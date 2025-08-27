use convex_client::leptos::Query;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FriendDetails {
    pub _id: String,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PendingFriendRequest {
    pub _id: String,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "senderName")]
    pub sender_name: String,
    #[serde(rename = "senderImageUrl")]
    pub sender_image_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SentFriendRequest {
    pub _id: String,
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
    #[serde(rename = "receiverImageUrl")]
    pub receiver_image_url: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetFriends {
    pub auth: i64,
}

impl Query<Vec<FriendDetails>> for GetFriends {
    fn name(&self) -> String {
        "friends:getFriends".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetPendingFriendRequests {
    pub auth: i64,
}

impl Query<Vec<PendingFriendRequest>> for GetPendingFriendRequests {
    fn name(&self) -> String {
        "friends:getPendingFriendRequests".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetSentFriendRequests {
    pub auth: i64,
}

impl Query<Vec<SentFriendRequest>> for GetSentFriendRequests {
    fn name(&self) -> String {
        "friends:getSentFriendRequests".to_string()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetFriendshipStatus {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub auth: i64,
}

impl Query<String> for GetFriendshipStatus {
    fn name(&self) -> String {
        "friends:getFriendshipStatus".to_string()
    }
}

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
