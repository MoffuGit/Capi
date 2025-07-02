use std::sync::Arc;

use dashmap::DashMap;
use futures::channel::mpsc;
use futures::SinkExt;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

use api::convex::{sync, Query, QueryResponse, SyncRequest, SyncResponse};

#[derive(Debug, Default, Clone)]
struct SyncChannels {
    channels: Arc<DashMap<Query, mpsc::Sender<QueryResponse>>>,
}

#[derive(Debug, Clone)]
pub struct SyncContext {
    pub tx: mpsc::Sender<(SyncRequest, Option<mpsc::Sender<QueryResponse>>)>,
}

#[component]
pub fn SyncProvider(children: Children) -> impl IntoView {
    use futures::{channel::mpsc, StreamExt};
    let channels = SyncChannels::default();
    let (mut ws_tx, ws_rx) = mpsc::channel(1);
    let (tx, mut rx) = mpsc::channel(100);
    if cfg!(feature = "hydrate") {
        let channels_clone = channels.clone();
        spawn_local(async move {
            match sync(ws_rx.into()).await {
                Ok(mut messages) => {
                    while let Some(Ok(SyncResponse { query, res })) = messages.next().await {
                        if let Some(mut sender) = channels_clone.channels.get_mut(&query) {
                            let _ = sender.value_mut().send(res).await;
                        }
                    }
                }
                Err(e) => leptos::logging::warn!("{e}"),
            }
        });

        spawn_local(async move {
            while let Some((request, receiver)) = rx.next().await {
                match &request {
                    SyncRequest::Subscribe(query) => {
                        if let Some(receiver) = receiver {
                            channels.channels.insert(query.clone(), receiver);
                        }
                    }
                    SyncRequest::Unsubscribe(query) => {
                        channels.channels.remove(query);
                    }
                }
                let _ = ws_tx.send(Ok(request)).await;
            }
        })
    }
    view! {
        <Provider value=SyncContext {
            tx
        }>
            {children()}
        </Provider>
    }
}
