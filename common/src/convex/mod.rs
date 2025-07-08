use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub name: String,
    pub image_url: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_creationTime")]
    pub creation_time: f64,
    pub user: String,
    pub server: String,
    pub roles: Vec<String>,
    pub name: String,
    pub image_url: Option<String>,
    #[serde(rename = "lastVisitedChannel")]
    pub last_visited_channel: Option<String>,
    pub online: bool,
    #[serde(rename = "mostImportantRole")]
    pub most_important_role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
