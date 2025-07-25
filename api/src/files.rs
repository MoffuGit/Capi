use convex_client::leptos::Mutation;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GenerateUploadUrl {
    pub auth: i64,
}

impl Mutation for GenerateUploadUrl {
    fn name(&self) -> String {
        "files:generateUploadUrl".to_string()
    }

    type Output = Option<String>;
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct SetImageUrl {
    pub auth: i64,
    #[serde(rename = "storageId")]
    pub storage_id: String,
}

impl Mutation for SetImageUrl {
    fn name(&self) -> String {
        "user:setImageUrl".to_string()
    }

    type Output = Option<String>;
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct RemoveUserImage {
    pub auth: i64,
}

impl Mutation for RemoveUserImage {
    fn name(&self) -> String {
        "user:removeUserImage".to_string()
    }

    type Output = Option<bool>;
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct RemoveUserBanner {
    pub auth: i64,
}

impl Mutation for RemoveUserBanner {
    fn name(&self) -> String {
        "user:removeUserBanner".to_string()
    }

    type Output = Option<bool>;
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct SetBannerUrl {
    pub auth: i64,
    #[serde(rename = "storageId")]
    pub storage_id: String,
}

impl Mutation for SetBannerUrl {
    fn name(&self) -> String {
        "user:setBannerUrl".to_string()
    }

    type Output = Option<String>;
}
