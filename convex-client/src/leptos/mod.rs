#![allow(clippy::new_ret_no_self)]
mod json;
mod subscription;
mod worker;

use crate::leptos::worker::worker;
use async_trait::async_trait;
use leptos::prelude::*;
use leptos::task::spawn_local_scoped_with_cancellation;
use std::sync::Arc;

use convex_sync_types::{AuthenticationToken, UdfPath, UserIdentityAttributes};
use futures::channel::{mpsc, oneshot};
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use url::Url;
use wasm_bindgen_futures::spawn_local;

use crate::base::BaseConvexClient;
use crate::base::query_result::{FunctionResult, QueryResults};
use crate::websocket::{ProtocolResponse, SyncProtocol, WebSocketState};
use serde_json::Value;

use self::json::convex_json;
use self::subscription::{QuerySetSubscription, QuerySubscription};
use self::worker::{
    ActionRequest, AuthenticateRequest, ClientRequest, MutationRequest, SubscribeRequest,
};

pub struct ConvexClient {
    shutdown_listen: Arc<oneshot::Sender<()>>,
    request_sender: mpsc::UnboundedSender<ClientRequest>,
    watch_receiver: async_broadcast::Receiver<QueryResults>,
}

impl Clone for ConvexClient {
    fn clone(&self) -> Self {
        Self {
            shutdown_listen: self.shutdown_listen.clone(),
            request_sender: self.request_sender.clone(),
            watch_receiver: self.watch_receiver.new_receiver(),
        }
    }
}
#[cfg(feature = "hydrate")]
impl ConvexClient {
    /// Constructs a new client for communicating with `deployment_url`.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(deployment_url: &str) -> anyhow::Result<Self> {
        ConvexClient::new_from_builder(ConvexClientBuilder::new(deployment_url))
    }

    #[doc(hidden)]
    pub fn new_from_builder(builder: ConvexClientBuilder) -> anyhow::Result<Self> {
        use crate::websocket::client::WebSocketManager;
        let client_id = builder
            .client_id
            .unwrap_or_else(|| format!("rust-{}", "unknown"));
        let ws_url = deployment_to_ws_url(builder.deployment_url.as_str().try_into()?)?;

        let (response_sender, response_receiver) = mpsc::channel::<ProtocolResponse>(1);
        let (request_sender, request_receiver) = mpsc::unbounded();

        let (mut watch_sender, watch_receiver) = async_broadcast::broadcast(1);
        watch_sender.set_overflow(true);

        let (shutdown_sender, shutdown_listen) = oneshot::channel();

        let base_client = BaseConvexClient::new();

        spawn_local(async move {
            let protocol = WebSocketManager::open(
                ws_url,
                response_sender,
                builder.on_state_change,
                client_id.as_str(),
            )
            .await
            .expect("should open the websocket");

            worker(
                response_receiver,
                request_receiver,
                watch_sender,
                base_client,
                shutdown_listen,
                protocol,
            )
            .await;
        });
        let client = ConvexClient {
            shutdown_listen: Arc::new(shutdown_sender),
            request_sender,
            watch_receiver,
        };
        Ok(client)
    }
}

impl ConvexClient {
    pub async fn subscribe(
        &mut self,
        name: &str,
        args: Value,
    ) -> anyhow::Result<QuerySubscription> {
        let (tx, rx) = oneshot::channel();

        let udf_path = name.parse()?;
        let request = SubscribeRequest { udf_path, args };

        self.request_sender
            .send(ClientRequest::Subscribe(
                request,
                tx,
                self.request_sender.clone(),
            ))
            .await?;

        let res = rx.await?;
        Ok(res)
    }

    pub async fn query(&mut self, name: &str, args: Value) -> anyhow::Result<FunctionResult> {
        Ok(self
            .subscribe(name, args)
            .await?
            .next()
            .await
            .expect("INTERNAL BUG: Convex Client dropped prematurely."))
    }

    /// Perform a mutation `name` with `args` and return a future
    /// containing the return value of the mutation once it completes.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let result = client.mutation("sendMessage", maplit::btreemap!{
    ///     "body".into() => "Let it be.".into(),
    ///     "author".into() => "The Beatles".into(),
    /// }).await?;
    /// println!("{result:?}");
    /// # Ok(())
    /// # }
    pub async fn mutation(&mut self, name: &str, args: Value) -> anyhow::Result<FunctionResult> {
        let (tx, rx) = oneshot::channel();

        let udf_path: UdfPath = name.parse()?;
        let request = MutationRequest { udf_path, args };

        self.request_sender
            .send(ClientRequest::Mutation(request, tx))
            .await?;

        let res = rx.await?;
        Ok(res.await?)
    }

    /// Perform an action `name` with `args` and return a future
    /// containing the return value of the action once it completes.
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let result = client.action("sendGif", maplit::btreemap!{
    ///     "body".into() => "Tatooine Sunrise.".into(),
    ///     "author".into() => "Luke Skywalker".into(),
    /// }).await?;
    /// println!("{result:?}");
    /// # Ok(())
    /// # }
    pub async fn action(&mut self, name: &str, args: Value) -> anyhow::Result<FunctionResult> {
        let (tx, rx) = oneshot::channel();

        let udf_path: UdfPath = name.parse()?;
        let request = ActionRequest { udf_path, args };

        self.request_sender
            .send(ClientRequest::Action(request, tx))
            .await?;

        let res = rx.await?;
        Ok(res.await?)
    }

    /// Get a consistent view of the results of multiple queries (query set).
    ///
    /// Returns a [`QuerySetSubscription`] which
    /// implements [`Stream`]<[`QueryResults`]>.
    /// Each item in the stream contains a consistent view
    /// of the results of all the queries in the query set.
    ///
    /// Queries can be added to the query set via [`ConvexClient::subscribe`].
    /// Queries can be removed from the query set via dropping the
    /// [`QuerySubscription`] token returned by [`ConvexClient::subscribe`].
    ///
    ///
    /// [`QueryResults`] is a copy-on-write mapping from [`SubscriberId`] to
    /// its latest result [`Value`].
    ///
    /// ```no_run
    /// # use convex::ConvexClient;
    /// # use futures::StreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> anyhow::Result<()> {
    /// let mut client = ConvexClient::new("https://cool-music-123.convex.cloud").await?;
    /// let mut watch = client.watch_all();
    /// let sub1 = client.subscribe("listMessages", maplit::btreemap!{
    ///     "channel".into() => 1.into(),
    /// }).await?;
    /// let sub2 = client.subscribe("listMessages", maplit::btreemap!{
    ///     "channel".into() => 1.into(),
    /// }).await?;
    /// # Ok(())
    /// # }
    pub fn watch_all(&self) -> QuerySetSubscription {
        QuerySetSubscription::new(self.watch_receiver.new_receiver())
    }

    /// Set auth for use when calling Convex functions.
    ///
    /// Set it with a token that you get from your auth provider via their login
    /// flow. If `None` is passed as the token, then auth is unset (logging
    /// out).
    pub async fn set_auth(&mut self, token: Option<String>) {
        let req = AuthenticateRequest {
            token: match token {
                None => AuthenticationToken::None,
                Some(token) => AuthenticationToken::User(token),
            },
        };
        self.request_sender
            .send(ClientRequest::Authenticate(Box::new(req)))
            .await
            .expect("INTERNAL BUG: Worker has gone away");
    }

    /// Set admin auth for use when calling Convex functions as a deployment
    /// admin. Not typically required.
    ///
    /// You can get a deploy_key from the Convex dashboard's deployment settings
    /// page. Deployment admins can act as users as part of their
    /// development flow to see how a function would act.
    #[doc(hidden)]
    pub async fn set_admin_auth(
        &mut self,
        deploy_key: String,
        acting_as: Option<UserIdentityAttributes>,
    ) {
        let req = AuthenticateRequest {
            token: AuthenticationToken::Admin(deploy_key, acting_as),
        };
        self.request_sender
            .send(ClientRequest::Authenticate(Box::new(req)))
            .await
            .expect("INTERNAL BUG: Worker has gone away");
    }
}

fn deployment_to_ws_url(mut deployment_url: Url) -> anyhow::Result<Url> {
    let ws_scheme = match deployment_url.scheme() {
        "http" | "ws" => "ws",
        "https" | "wss" => "wss",
        scheme => anyhow::bail!("Unknown scheme {scheme}. Expected http or https."),
    };
    deployment_url
        .set_scheme(ws_scheme)
        .expect("Scheme not supported");
    deployment_url.set_path("api/sync");
    Ok(deployment_url)
}

/// A builder for creating a [`ConvexClient`] with custom configuration.
pub struct ConvexClientBuilder {
    deployment_url: String,
    client_id: Option<String>,
    on_state_change: Option<mpsc::Sender<WebSocketState>>,
}

#[cfg(feature = "hydrate")]
impl ConvexClientBuilder {
    pub fn new(deployment_url: &str) -> Self {
        Self {
            deployment_url: deployment_url.to_string(),
            client_id: None,
            on_state_change: None,
        }
    }

    pub fn with_client_id(mut self, client_id: &str) -> Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    pub fn with_on_state_change(mut self, on_state_change: mpsc::Sender<WebSocketState>) -> Self {
        self.on_state_change = Some(on_state_change);
        self
    }
    pub async fn build(self) -> anyhow::Result<ConvexClient> {
        ConvexClient::new_from_builder(self)
    }
}

#[component]
pub fn ConvexProviderWithAuth(
    children: Children,
    access_token: impl Fn() -> Option<String> + Send + Sync + 'static,
) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        use leptos::task::spawn_local;

        let access_token = Memo::new(move |_| access_token());

        let client = ConvexClient::new("https://quick-cardinal-805.convex.cloud")
            .expect("should provide the convex client");
        let client_clone = client.clone();
        Effect::new(move |_| {
            let mut client = client_clone.clone();
            let access_token = access_token.get();
            spawn_local(async move {
                client.set_auth(access_token).await;
            });
        });
        provide_context(client);
    }

    view! {
        {children()}
    }
}

#[component]
pub fn ConvexProvider(children: Children) -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        let client = ConvexClient::new("https://quick-cardinal-805.convex.cloud")
            .expect("should provide the convex client");
        provide_context(client);
    }
    view! {
        {children()}
    }
}

pub trait Query<F: DeserializeOwned + Send + Sync + 'static> {
    fn name(&self) -> String;
    fn args(&self) -> anyhow::Result<Value>
    where
        Self: serde::Serialize,
    {
        let mut value = serde_json::to_value(self)?;
        convex_json(&mut value);
        Ok(value)
    }
}

pub struct UseQuery;

impl UseQuery {
    pub fn new<F, Q>(
        query: impl Fn() -> Option<Q> + Send + Sync + 'static,
    ) -> ReadSignal<Option<Result<F, String>>>
    where
        F: DeserializeOwned + PartialEq + Clone + Send + Sync + 'static,
        Q: Query<F> + Serialize + Send + Sync + 'static + PartialEq + Clone,
    {
        let source = Memo::new(move |_| query());

        let (query_signal, set_query_signal) = signal(None);

        Effect::new(move |_| {
            if let Some(mut client) = use_context::<ConvexClient>() {
                if let Some(query) = source.get() {
                    set_query_signal.set(None);
                    let name_clone = query.name().clone();
                    let args_value = match query.args() {
                        Ok(val) => val,
                        Err(e) => {
                            set_query_signal(Some(Err(format!(
                                "Failed to serialize query arguments: {e}"
                            ))));
                            return;
                        }
                    };

                    spawn_local_scoped_with_cancellation(async move {
                        match client.subscribe(&name_clone, args_value).await {
                            Ok(sub) => {
                                let mut sub_stream = sub.map(|result| match result {
                                    FunctionResult::Value(value) => {
                                        match serde_json::from_value::<F>(value) {
                                            Err(err) => Err(format!("{err}")),
                                            Ok(value) => Ok(value),
                                        }
                                    }
                                    FunctionResult::ErrorMessage(err) => Err(err),
                                    FunctionResult::ConvexError(convex_error) => {
                                        Err(format!("{convex_error:?}"))
                                    }
                                });
                                while let Some(result) = sub_stream.next().await {
                                    let prev_value = query_signal.get_untracked();
                                    if prev_value.is_none()
                                        || prev_value.is_some_and(|prev| prev != result)
                                    {
                                        set_query_signal(Some(result));
                                    }
                                }
                            }
                            Err(e) => {
                                set_query_signal(Some(Err(format!("Failed to subscribe: {e}"))));
                            }
                        }
                    });
                } else {
                    set_query_signal(None);
                }
            } else {
                set_query_signal(None);
            }
        });
        query_signal
    }

    pub fn with_preloaded<F, Q>(
        query: impl Fn() -> Option<Q> + Send + Sync + 'static,
        preloaded: F,
    ) -> Signal<Option<Result<F, String>>>
    where
        F: DeserializeOwned + PartialEq + Clone + Send + Sync + 'static,
        Q: Query<F> + Serialize + Send + Sync + 'static + PartialEq + Clone,
    {
        let query_signal = Self::new(query);

        Signal::derive(move || {
            if query_signal.get().is_none() {
                Some(Ok(preloaded.clone()))
            } else {
                query_signal()
            }
        })
    }
}

pub struct UseMutation;

impl UseMutation {
    pub fn new<M>() -> Action<M, Result<M::Output, String>>
    where
        M: Mutation + Clone,
    {
        UseMutation::with_fn(move |(mutation, client): (&M, &mut ConvexClient)| {
            let mutation = mutation.to_owned();
            let mut client = client.to_owned();
            async move { mutation.run(&mut client).await }
        })
    }

    pub fn with_fn<I, O, F, Fu>(action_fn: F) -> Action<I, O>
    where
        F: Fn((&I, &mut ConvexClient)) -> Fu + Send + Sync + 'static,
        Fu: Future<Output = O> + Send + 'static,
        I: Send + Sync + 'static,
        O: Send + Sync + 'static,
    {
        let client = use_context::<ConvexClient>();
        Action::new(move |input: &I| {
            let mut client = client.clone().unwrap();
            action_fn((input, &mut client))
        })
    }

    pub fn with_local_fn<I, O, F, Fu>(action_fn: F) -> Action<I, O>
    where
        F: Fn((&I, &mut ConvexClient)) -> Fu + 'static,
        Fu: Future<Output = O> + 'static,
        I: 'static,
        O: 'static,
    {
        let client = use_context::<ConvexClient>();
        Action::new_local(move |input: &I| {
            let mut client = client.clone().unwrap();
            action_fn((input, &mut client))
        })
    }
}

#[async_trait]
pub trait Mutation: Serialize + Send + Sync + 'static {
    type Output: DeserializeOwned + Send + Sync + 'static;
    fn name(&self) -> String;
    fn args(&self) -> anyhow::Result<Value>
    where
        Self: serde::Serialize,
    {
        let mut value = serde_json::to_value(self)?;
        convex_json(&mut value);
        Ok(value)
    }
    async fn run(&self, client: &mut ConvexClient) -> Result<Self::Output, String> {
        match client
            .mutation(&self.name(), self.args().unwrap_or_default())
            .await
        {
            Ok(FunctionResult::Value(value)) => {
                match serde_json::from_value::<Self::Output>(value) {
                    Err(err) => Err(format!("{err}")),
                    Ok(value) => Ok(value),
                }
            }
            Ok(FunctionResult::ErrorMessage(err)) => Err(err),
            Ok(FunctionResult::ConvexError(convex_error)) => Err(format!("{convex_error:?}")),
            Err(e) => Err(format!("Mutation failed: {e}")),
        }
    }
}

// pub fn use_mutation<Q, F>() -> LeptosAction<Q, Result<F, String>>
// where
//     Q: Mutation + Send + Sync + Serialize + 'static + Clone,
//     F: DeserializeOwned + Send + Sync + 'static,
// {
//     LeptosAction::new(|mutation: &Q| {
//         let mutation = mutation.to_owned();
//         async move {
//             let mut client = use_context::<ConvexClient>().expect("should acces the convex client");
//             match client
//                 .mutation(&mutation.name(), mutation.args().expect("you know"))
//                 .await
//             {
//                 Ok(FunctionResult::Value(value)) => match serde_json::from_value::<F>(value) {
//                     Err(err) => Err(format!("{err}")),
//                     Ok(value) => Ok(value),
//                 },
//                 Ok(FunctionResult::ErrorMessage(err)) => Err(err),
//                 Ok(FunctionResult::ConvexError(convex_error)) => Err(format!("{convex_error:?}")),
//                 Err(_) => Err("fuck".into()),
//             }
//         }
//     })
// }
//
// pub fn use_action<F: DeserializeOwned + Send + Sync + 'static>(
//     action: impl Mutation + Serialize,
// ) -> (ReadSignal<Option<Result<F, String>>>, ReadSignal<bool>) {
//     let name = action.name();
//     let args = action.args();
//     let (query, set_query) = signal(None);
//     let (loading, set_loading) = signal(false);
//
//     Effect::new(move |_| {
//         if let Some(mut client) = use_context::<ConvexClient>() {
//             let name = name.clone();
//             spawn_local_scoped_with_cancellation(async move {
//                 set_loading(true);
//                 if let Ok(mutation) = client.action(&name, BTreeMap::new()).await {
//                     set_loading(false);
//                     let result = match mutation {
//                         FunctionResult::Value(value) => match serde_json::from_value::<F>(value) {
//                             Err(err) => Err(format!("{err}")),
//                             Ok(value) => Ok(value),
//                         },
//                         FunctionResult::ErrorMessage(err) => Err(err),
//                         FunctionResult::ConvexError(convex_error) => {
//                             Err(format!("{convex_error:?}"))
//                         }
//                     };
//                     set_query(Some(result));
//                 }
//             });
//         }
//     });
//
//     (query, loading)
// }
//
//
// pub trait Action<F: DeserializeOwned + Send + Sync + 'static> {
//     fn name(&self) -> String;
//     fn args(&self) -> anyhow::Result<Value>
//     where
//         Self: serde::Serialize,
//     {
//         Ok(serde_json::to_value(self)?)
//     }
// }
//
