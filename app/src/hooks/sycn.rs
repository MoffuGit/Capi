use std::fmt::Debug;

use api::convex::{Query, QueryResponse, SyncRequest};
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use leptos::logging::log;
use leptos::prelude::{on_cleanup, use_context, RwSignal, Set};
use leptos::reactive::spawn_local;
use serde::Deserialize;

use crate::sync::SyncContext;

pub struct SyncSignal<T: for<'a> Deserialize<'a>> {
    pub signal: RwSignal<Option<T>>,
}

impl<T: for<'a> Deserialize<'a> + Send + Sync + Debug + 'static> SyncSignal<T> {
    pub fn new<Q: Into<Query> + Send + Sync + 'static>(query: Q) -> Self {
        let query = query.into();
        let (tx, mut rx) = mpsc::channel(100);
        let signal = RwSignal::new(None);
        let sync: SyncContext = use_context().expect("should acces the sync context");
        if cfg!(feature = "hydrate") {
            let query_clone = query.clone();
            let mut tx_clone = sync.tx.clone();
            spawn_local(async move {
                let _ = tx_clone
                    .send((SyncRequest::Subscribe(query_clone), Some(tx)))
                    .await;
            });

            spawn_local(async move {
                while let Some(msg) = rx.next().await {
                    if let QueryResponse::Update(value) = msg {
                        let new_value = serde_json::from_value::<T>(value);
                        if let Ok(new_value) = new_value {
                            signal.set(Some(new_value));
                        } else {
                            log!("we cant deserialize to the desired value: {new_value:?}")
                        }
                    } else {
                        log!("{msg:?}");
                    }
                }
            });
            let mut tx_clone = sync.tx.clone();
            on_cleanup(move || {
                spawn_local(async move {
                    let _ = tx_clone
                        .send((SyncRequest::Unsubscribe(query.clone()), None))
                        .await;
                })
            });
        }
        SyncSignal { signal }
    }
}
