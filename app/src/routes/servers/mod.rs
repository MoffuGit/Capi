mod components;

use api::convex::{sync, Query, SyncRequest};
use futures::SinkExt;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use serde_json::json;

use crate::components::primitives::common::Orientation;
use crate::components::ui::divider::Separator;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};

use self::components::sidebar::SideBar;

#[component]
pub fn Servers() -> impl IntoView {
    use futures::{channel::mpsc, StreamExt};
    let (mut tx, rx) = mpsc::channel(1);

    // we'll only listen for websocket messages on the client
    if cfg!(feature = "hydrate") {
        spawn_local(async move {
            match sync(rx.into()).await {
                Ok(mut messages) => {
                    while let Some(msg) = messages.next().await {
                        leptos::logging::log!("{:?}", msg);
                    }
                }
                Err(e) => leptos::logging::warn!("{e}"),
            }
        });
    }

    let mut tx_clone = tx.clone();
    if cfg!(feature = "hydrate") {
        spawn_local(async move {
            let _ = tx_clone
                .send(Ok(SyncRequest::Subscribe(Query {
                    name: "task:get".to_string(),
                    args: json! {{}},
                })))
                .await;
        })
    }

    view! {
        <SidebarProvider style="--sidebar-width: 350px;">
            <SideBar/>
            <SidebarInset>
                <header class="bg-background sticky top-0 flex shrink-0 items-center gap-2 border-b p-4">
                    <SidebarTrigger class="-ml-1" />
                    <Separator
                        orientation=Orientation::Vertical
                        class="mr-2 data-[orientation=vertical]:h-4"
                    />
                </header>
            </SidebarInset>
        </SidebarProvider>
    }
}
