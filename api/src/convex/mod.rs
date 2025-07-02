#[cfg(feature = "ssr")]
mod subscripton;

use futures::channel::mpsc;
use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server;
use leptos::server_fn::codec::JsonEncoding;
use leptos::server_fn::{BoxedStream, Websocket};
use serde::{Deserialize, Serialize};

use serde_json::Value as JsonValue;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum QueryResponse {
    Update(JsonValue),
    Deleted(JsonValue),
    Added(JsonValue),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Query {
    pub name: String,
    pub args: JsonValue,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SyncResponse {
    query: Query,
    res: QueryResponse,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SyncRequest {
    Subscribe(Query),
    Unsubscribe(Query),
}

#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
pub async fn sync(
    request: BoxedStream<SyncRequest, ServerFnError>,
) -> Result<BoxedStream<SyncResponse, ServerFnError>, ServerFnError> {
    use self::subscripton::SubscriptionManager;
    use common::state::convex;
    let (tx, rx) = mpsc::channel(1);
    let client = convex()?;
    let mut subscription_manager = SubscriptionManager::new(client);

    log!("at least we got here");
    tokio::spawn(async move { subscription_manager.run_worker(request, tx).await });

    Ok(rx.into())
}
