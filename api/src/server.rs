use common::convex::{Member, Role, Server};
use convex_client::leptos::Query;
use leptos::server;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerData {
    pub server: Server,
    pub member: Member,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GetServers {
    pub auth: i64,
}

impl Query<Vec<ServerData>> for GetServers {
    fn name(&self) -> String {
        "user:getServers".into()
    }
}

#[server]
pub async fn preload_server_data() -> Result<Vec<ServerData>, ServerFnError> {
    use auth::auth;
    use common::state::convex;
    let auth = auth().await?;
    let mut client = convex()?;
    if let Some(user) = auth.current_user {
        client
            .query(GetServers {
                auth: user.user().id,
            })
            .await
            .map_err(|err| ServerFnError::new(format!("{err}")))
    } else {
        Ok(vec![])
    }
}
