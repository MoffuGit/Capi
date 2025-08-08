use std::str::FromStr as _;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::{Display, EnumString};

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
    pub description: Option<String>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: Option<String>,
    #[serde(rename = "type")]
    pub _type: ServerType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ServerType {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ChannelType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub online: bool,
    #[serde(rename = "mostImportantRole")]
    pub most_important_role: Option<String>,
}

#[derive(
    Debug,
    Copy,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    strum_macros::Display,
    strum_macros::EnumIter,
    Eq,
    Hash,
)]
pub enum PresenceStatus {
    #[serde(rename = "Online")]
    Online,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "NotDisturb")]
    #[strum(to_string = "Not Disturb")]
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
    pub emoji: String,
    pub count: f64,
    #[serde(rename = "hasReacted")]
    pub has_reacted: bool,
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
    #[serde(rename = "storageId")]
    pub storage_id: String,
    pub url: Option<String>,
    pub metadata: Option<FileMetaData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FileMetaData {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    #[serde(rename = "contentType")]
    pub content_type: FileType,
    pub sha256: String,
    pub size: f64,
}

#[derive(Debug, Display, EnumString, PartialEq, Default, Clone, Copy)]
pub enum FileType {
    #[strum(serialize = "image/jpeg", serialize = "image/jpg")]
    Jpeg,
    #[strum(serialize = "image/png")]
    Png,
    #[strum(serialize = "image/gif")]
    Gif,
    #[strum(serialize = "image/webp")]
    Webp,
    #[strum(serialize = "application/pdf")]
    Pdf,
    #[strum(serialize = "text/plain")]
    Text,
    #[strum(serialize = "application/msword")]
    Doc,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.wordprocessingml.document")]
    Docx,
    #[strum(serialize = "application/vnd.ms-excel")]
    Xls,
    #[strum(serialize = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")]
    Xlsx,
    #[strum(serialize = "application/zip")]
    Zip,
    #[strum(serialize = "audio/mpeg")]
    Mp3,
    #[strum(serialize = "audio/wav")]
    Wav,
    #[strum(serialize = "video/mp4")]
    Mp4,
    #[strum(serialize = "video/webm")]
    Webm,
    #[strum(serialize = "application/json")]
    Json,
    #[strum(serialize = "text/csv")]
    Csv,
    #[strum(serialize = "text/html")]
    Html,
    #[default]
    Unknown,
}

// Implement Serialize for FileType using its Display trait
impl Serialize for FileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Implement Deserialize for FileType using its EnumString trait
impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FileType::from_str(&s).map_err(Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelMessage {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub channel: String,
    pub sender: String,
    #[serde(rename = "referencedMessage")]
    pub referenced_message: Option<Box<ChannelMessage>>,
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
