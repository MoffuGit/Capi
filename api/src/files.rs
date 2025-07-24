use convex_client::leptos::Mutation;
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GenerateUploadUrl {
    auth: i64,
}

impl Mutation for GenerateUploadUrl {
    fn name(&self) -> String {
        "files:generateUploadUrl".to_string()
    }

    type Output = Option<String>;
}
