use std::collections::{BTreeMap, HashMap};

use convex::{ConvexClient, Value as ConvexValue};
use futures::future::BoxFuture;
use futures::stream::FuturesUnordered;
use futures::{FutureExt, SinkExt, StreamExt};
use leptos::logging::log;
use leptos::prelude::ServerFnError;
use leptos::server_fn::BoxedStream;

use futures::channel::mpsc;
use serde_json::Value as JsonValue;
use tokio::sync::watch;

use crate::convex::QueryResponse;

use super::{Query, SyncRequest, SyncResponse};

#[derive(Copy, Clone)]
enum SubscriptionState {
    Valid,
    Invalid,
}

fn convert_json_value_to_convex_value(json_val: JsonValue) -> ConvexValue {
    match json_val {
        JsonValue::Null => ConvexValue::Null,
        JsonValue::Bool(b) => ConvexValue::Boolean(b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                ConvexValue::Int64(i)
            } else if let Some(f) = n.as_f64() {
                ConvexValue::Float64(f)
            } else {
                ConvexValue::Float64(0.0)
            }
        }
        JsonValue::String(s) => ConvexValue::String(s),
        JsonValue::Array(arr) => ConvexValue::Array(
            arr.into_iter()
                .map(convert_json_value_to_convex_value)
                .collect(),
        ),
        JsonValue::Object(obj) => {
            let mut btree_map = BTreeMap::new();
            for (k, v) in obj {
                btree_map.insert(k, convert_json_value_to_convex_value(v));
            }
            ConvexValue::Object(btree_map)
        }
    }
}

pub fn json_to_convex(json: JsonValue) -> BTreeMap<String, ConvexValue> {
    match json {
        JsonValue::Object(obj) => {
            let mut btree_map = BTreeMap::new();
            for (key, value) in obj {
                btree_map.insert(key, convert_json_value_to_convex_value(value));
            }
            btree_map
        }
        _ => BTreeMap::new(),
    }
}

pub struct SubscriptionManager {
    client: ConvexClient,
    subscriptions: HashMap<Query, Subscription>,
    closed_subscription: FuturesUnordered<BoxFuture<'static, Query>>,
}

struct Subscription {
    valid_tx: watch::Sender<SubscriptionState>,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        _ = self.valid_tx.send(SubscriptionState::Invalid);
    }
}

#[allow(clippy::new_without_default)]
impl SubscriptionManager {
    pub fn new(client: ConvexClient) -> Self {
        SubscriptionManager {
            subscriptions: HashMap::new(),
            client,
            closed_subscription: FuturesUnordered::new(),
        }
    }

    pub fn remove(&mut self, query: Query) {
        self.subscriptions.remove(&query);
    }

    pub async fn subscribe(
        &mut self,
        query: Query,
        mut tx: mpsc::Sender<Result<SyncResponse, ServerFnError>>,
    ) {
        if self.subscriptions.contains_key(&query) {
            return;
        }
        let (valid_tx, mut valid_rx) = watch::channel(SubscriptionState::Valid);
        match self
            .client
            .subscribe(&query.name, json_to_convex(query.args.clone()))
            .await
        {
            Ok(mut sub) => {
                self.subscriptions.insert(
                    query.clone(),
                    Subscription {
                        valid_tx: valid_tx.clone(),
                    },
                );

                let query_for_closed_subscription = query.clone();
                self.closed_subscription.push(
                    async move {
                        valid_tx.closed().await;
                        query_for_closed_subscription
                    }
                    .boxed(),
                );

                let query_for_spawned_task = query.clone();
                let _ = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = valid_rx.changed().fuse() => {
                                log!("the rx is invalid, it should end the spawn");
                                break;
                            },
                            value = sub.next().fuse() => {
                                match value {
                                    Some(value) => {
                                        match value {
                                            convex::FunctionResult::Value(value) => {
                                                let _ = tx.send(Ok(SyncResponse {
                                                    query: query_for_spawned_task.clone(),
                                                    res:QueryResponse::Update(value.into())
                                                })).await;
                                            },
                                            convex::FunctionResult::ErrorMessage(err) => {
                                                log!("error msg {err}");
                                                break;
                                            },
                                            convex::FunctionResult::ConvexError(err) => {
                                                log!("convex error msg {err}");
                                                break;
                                            },
                                        }
                                    },
                                    None => {
                                        log!("what should happen here?")
                                    },
                                }
                            }
                        }
                    }
                    log!("closed spawn");
                })
                .await;
            }
            Err(_err) => {
                let _ = tx
                    .send(Err(ServerFnError::new(
                        "We area having troubles receiving this information",
                    )))
                    .await;
            }
        }
    }

    pub async fn run_worker(
        &mut self,
        mut rx: BoxedStream<SyncRequest, ServerFnError>,
        tx: mpsc::Sender<Result<SyncResponse, ServerFnError>>,
    ) {
        log!("The subscription worker started");
        loop {
            futures::select_biased! {
                query = self.closed_subscription.select_next_some() => {
                    log!("");
                    self.remove(query);
                },
                request = rx.next().fuse() => {
                    log!("{request:?}");
                    match request {
                        Some(Ok(SyncRequest::Subscribe(query))) => {
                            self.subscribe(query, tx.clone()).await;
                        },
                        Some(Ok(SyncRequest::Unsubscribe(query))) => {
                            self.remove(query);
                        }
                        _ => {
                            break;
                        },
                    }
                }
            }
        }
        log!("The subscription worker ended");
    }
}
