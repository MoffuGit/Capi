use std::fmt::Debug;

use api::convex::{Query, QueryResponse};
use futures::channel::oneshot;
use futures::{SinkExt, StreamExt};
use leptos::prelude::{use_context, RwSignal, Set};
use leptos::task::spawn_local_scoped_with_cancellation;
use serde::Deserialize;

use crate::sync::SyncContext;

pub struct SyncSignal<T: for<'a> Deserialize<'a>> {
    pub signal: RwSignal<Option<T>>,
}

impl<T: for<'a> Deserialize<'a> + Send + Sync + Debug + 'static> SyncSignal<T> {
    pub fn new<Q: Into<Query> + Send + Sync + 'static>(query: Q) -> Self {
        let mut sync: SyncContext = use_context().expect("should acces the sync context");

        let query = query.into();

        let signal = RwSignal::new(None);
        if cfg!(feature = "hydrate") {
            spawn_local_scoped_with_cancellation(async move {
                let (tx, rx) = oneshot::channel();
                let _ = sync.tx.send((query, tx)).await;
                if let Ok((mut rx, value)) = rx.await {
                    signal.set(value.and_then(|value| serde_json::from_value(value).ok()));
                    while let Some(msg) = rx.next().await {
                        match msg {
                            QueryResponse::Update(value) => {
                                signal.set(serde_json::from_value(value).ok());
                            }
                            QueryResponse::Deleted(value) => todo!(),
                            QueryResponse::Added(value) => todo!(),
                        }
                    }
                }
            });
        }
        SyncSignal { signal }
    }
}
