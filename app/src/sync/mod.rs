use std::collections::HashMap;

use futures::channel::mpsc::Sender;
use futures::channel::{mpsc, oneshot};
use futures::future::LocalBoxFuture;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, SinkExt, StreamExt};
use leptos::context::Provider;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

use api::convex::{sync, Query, QueryResponse, SyncRequest, SyncResponse};
use leptos::server_fn::BoxedStream;
use leptos_dom::log;
use serde_json::Value;

#[derive(Clone)]
pub struct SyncChannel {
    sender: async_broadcast::Sender<QueryResponse>,
    value: Option<Value>,
}

pub struct SyncManager {
    channels: HashMap<Query, SyncChannel>,
    closed_queries: FuturesUnordered<LocalBoxFuture<'static, Query>>,
}

impl SyncManager {
    pub fn new() -> Self {
        SyncManager {
            channels: HashMap::new(),
            closed_queries: FuturesUnordered::new(),
        }
    }

    pub fn remove(&mut self, query: &Query) {
        self.channels.remove(query);
    }

    pub async fn subscribe(
        &mut self,
        query: Query,
        tx: oneshot::Sender<(async_broadcast::Receiver<QueryResponse>, Option<Value>)>,
    ) {
        if let Some(channel) = self.channels.get(&query) {
            let _ = tx.send((channel.sender.new_receiver(), channel.value.clone()));
        } else {
            let (sender, receiver) = async_broadcast::broadcast(100);
            let channel = SyncChannel {
                sender: sender.clone(),
                value: None,
            };
            let query_for_closed_subscription = query.clone();
            self.closed_queries.push(
                async move {
                    while !sender.clone().is_closed() {
                        let duration = std::time::Duration::from_millis(50);
                        gloo_timers::future::sleep(duration).await;
                    }
                    query_for_closed_subscription
                }
                .boxed_local(),
            );
            let _ = tx.send((receiver, channel.value.clone()));

            self.channels.insert(query, channel);
        }
    }

    pub async fn run_worker(
        &mut self,
        mut rx: mpsc::Receiver<(
            Query,
            oneshot::Sender<(async_broadcast::Receiver<QueryResponse>, Option<Value>)>,
        )>,
        mut ws_rx: BoxedStream<SyncResponse, ServerFnError>,
        mut ws_tx: Sender<Result<SyncRequest, ServerFnError>>,
    ) {
        loop {
            futures::select_biased! {
                query = self.closed_queries.select_next_some() => {
                    log!("removing query {query:?}");
                    self.remove(&query);
                    log!("query will get removed: {query:?}");
                    let _ = ws_tx.send(Ok(SyncRequest::Unsubscribe(query))).await;
                },
                response = ws_rx.next().fuse() => {
                    if let Some(Ok(SyncResponse { query, res })) = response {
                        if let Some(sender) = self.channels.get(&query) {
                            let _ = sender.sender.broadcast(res).await;
                        }
                    }
                }
                request = rx.next().fuse() => {
                    if let Some((query, tx)) = request {
                        log!("sub request for query {query:?}");
                        let _ = ws_tx.send(Ok(SyncRequest::Subscribe(query.clone()))).await;
                        self.subscribe(query, tx).await;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncContext {
    pub tx: mpsc::Sender<(
        Query,
        oneshot::Sender<(async_broadcast::Receiver<QueryResponse>, Option<Value>)>,
    )>,
}

#[component]
pub fn SyncProvider(children: Children) -> impl IntoView {
    let (ws_tx, ws_rx) = mpsc::channel(1);

    let (tx, rx) = mpsc::channel(100);

    let mut sync_manager = SyncManager::new();

    if cfg!(feature = "hydrate") {
        spawn_local(async move {
            match sync(ws_rx.into()).await {
                Ok(messages) => sync_manager.run_worker(rx, messages, ws_tx).await,
                Err(e) => leptos::logging::warn!("{e}"),
            };
        });
    }

    let sync_context = SyncContext { tx };

    view! {
        <Provider value=sync_context>
            {children()}
        </Provider>
    }
}
