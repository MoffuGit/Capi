use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    pub image_url: Option<String>,
    #[serde(rename = "defaultRole")]
    pub default_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    pub image_url: Option<String>,
    pub about: Option<String>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Option<ChannelType>,
    pub category: Option<String>,
    pub server: String,
    pub topic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    pub server: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Invitation {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub server: String,
    pub invitation: String,
}

fn deserialize_f64_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                // Already an i64, use it directly
                Ok(i)
            } else if let Some(f) = num.as_f64() {
                // It's an f64, check if it's a whole number and fits in i64
                Ok(f as i64)
            } else {
                Err(D::Error::custom(format!(
                    "expected a number convertible to i64, but found {num:?}"
                )))
            }
        }
        _ => Err(D::Error::custom(format!(
            "expected a number, found {value:?}"
        ))),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Hash, Eq)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    #[serde(deserialize_with = "deserialize_f64_to_i64")]
    creation_time: i64,
    pub user: String,
    pub server: String,
    pub roles: Vec<String>,
    pub name: String,
    pub image_url: Option<String>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: Option<String>,
    #[serde(rename = "lastVisitedChannel")]
    pub last_visited_channel: Option<String>,
    pub online: bool,
    #[serde(rename = "mostImportantRole")]
    pub most_important_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum_macros::Display)]
pub enum PresenceStatus {
    #[serde(rename = "Online")]
    Online,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "NotDisturb")]
    NotDisturb,
    #[serde(rename = "Invisible")]
    Invisible,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleActions {
    #[serde(rename = "canManageChannels")]
    pub can_manage_channels: bool,
    #[serde(rename = "canManageCategories")]
    pub can_manage_categories: bool,
    #[serde(rename = "canManageRoles")]
    pub can_manage_roles: bool,
    #[serde(rename = "canManageMembers")]
    pub can_manage_members: bool,
    #[serde(rename = "canManageServerSettings")]
    pub can_manage_server_settings: bool,
    #[serde(rename = "canCreateInvitation")]
    pub can_create_invitation: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Invitations {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub server: String,
    pub invitation: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Role {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub server: String,
    pub name: String,
    #[serde(rename = "isOwner")]
    pub is_owner: bool,
    #[serde(rename = "canBeDeleted")]
    pub can_be_deleted: bool,
    pub level: f64,
    pub actions: RoleActions,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Reaction {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub message: String,
    pub member: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Mention {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub message: String,
    pub member: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RoleMention {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub message: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Attachment {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub message: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelMessage {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub channel: String,
    pub sender: String,
    pub reference: Option<String>,
    pub content: String,
    pub pinned: bool,
    #[serde(rename = "mention_everyone")]
    pub mention_everyone: bool,
    #[serde(rename = "mention_roles")]
    pub mention_roles: Vec<String>,
    pub reactions: Vec<Reaction>,
    pub mentions: Vec<Mention>,
    #[serde(rename = "role_mentions")]
    pub role_mentions: Vec<RoleMention>,
    pub attachments: Vec<Attachment>,
}
