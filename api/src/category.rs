use common::convex::Category;
use convex_client::leptos::Query;
use leptos::server;
use serde::Serialize;
use server_fn::ServerFnError;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GetCategories {
    pub server: String,
}

impl Query<Vec<Category>> for GetCategories {
    fn name(&self) -> String {
        "server:getCategories".to_string()
    }
}

#[server]
pub async fn preload_categories(server: Option<String>) -> Result<Vec<Category>, ServerFnError> {
    use auth::auth;
    use common::state::convex;

    let auth = auth().await?;
    let mut client = convex()?;
    let Some(server) = server else {
        return Ok(vec![]);
    };
    if let Some(_user) = auth.current_user {
        client
            .query(GetCategories { server })
            .await
            .map_err(|err| ServerFnError::new(format!("{err}")))
    } else {
        Err(ServerFnError::new("you need to be auth"))
    }
}
