use common::convex::Channel;
use convex_client::leptos::Query;
use leptos::server;
use serde::Serialize;
use server_fn::ServerFnError;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GetChannels {
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl Query<Vec<Channel>> for GetChannels {
    fn name(&self) -> String {
        "server:getChannels".to_string()
    }
}

#[server]
pub async fn preload_channels(
    server: Option<String>,
    category: Option<String>,
) -> Result<Vec<Channel>, ServerFnError> {
    use auth::auth;
    use common::state::convex;

    let auth = auth().await?;
    let mut client = convex()?;
    let Some(server) = server else {
        return Ok(vec![]);
    };
    if let Some(_user) = auth.current_user {
        client
            .query(GetChannels { server, category })
            .await
            .map_err(|err| ServerFnError::new(format!("{err}")))
    } else {
        Err(ServerFnError::new("you need to be auth"))
    }
}
