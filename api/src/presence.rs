use common::convex::PresenceStatus;
use convex_client::leptos::Query;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUserStatus {
    #[serde(rename = "userId")]
    pub user: String,
}

impl Query<Option<PresenceStatus>> for GetUserStatus {
    fn name(&self) -> String {
        "presence:getStatus".to_string()
    }
}
