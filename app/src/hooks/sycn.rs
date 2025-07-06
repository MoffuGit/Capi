use std::fmt::Debug;

use api::convex::{Query, QueryResponse};
use futures::channel::oneshot;
use futures::{SinkExt, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local_scoped_with_cancellation;
use leptos_dom::log;
use serde::Deserialize;

use crate::sync::SyncContext;

#[derive(Debug)]
pub struct SyncSignal<T: for<'a> Deserialize<'a>> {
    pub signal: RwSignal<Option<T>>,
}

impl<T: for<'a> Deserialize<'a> + Send + Sync + Debug + 'static> SyncSignal<T> {
    pub fn new(query: Memo<Option<Query>>) -> Self {
        let sync: SyncContext = use_context().expect("should acces the sync context");

        let signal = RwSignal::new(None);

        Effect::new(move |_| {
            if let Some(query) = query.get() {
                log!("{:?}", query);
                let mut sync_tx = sync.tx.clone();
                spawn_local_scoped_with_cancellation(async move {
                    let (tx, rx) = oneshot::channel();
                    let _ = sync_tx.send((query, tx)).await;
                    if let Ok((mut rx, value)) = rx.await {
                        signal.set(value.and_then(|value| serde_json::from_value(value).ok()));
                        while let Some(msg) = rx.next().await {
                            log!("{msg:?}");
                            match msg {
                                QueryResponse::Update(value) => {
                                    match serde_json::from_value(value) {
                                        Ok(value) => signal.set(Some(value)),
                                        Err(err) => {
                                            log!("Deserializing error: {err:?}");
                                        }
                                    }
                                }
                                QueryResponse::Deleted(value) => todo!(),
                                QueryResponse::Added(value) => todo!(),
                            }
                        }
                    }
                });
            } else {
                signal.set(None);
            }
        });

        SyncSignal { signal }
    }
}
