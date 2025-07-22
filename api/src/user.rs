use common::convex::User;
use convex_client::leptos::Query;
use leptos::server;
use serde::Serialize;
use server_fn::ServerFnError;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetUser {
    pub auth: i64,
}

impl Query<Option<User>> for GetUser {
    fn name(&self) -> String {
        "user:getUser".to_string()
    }
}

#[server]
pub async fn preload_user() -> Result<Option<User>, ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let auth = auth().await?;
    let mut client = convex()?;
    if let Some(user) = auth.current_user {
        let id = user.user().id;
        client
            .query(GetUser { auth: id })
            .await
            .map_err(|err| ServerFnError::new(format!("{err}")))
    } else {
        Ok(None)
    }
}
