mod components;
pub mod server;

use api::user::GetUser;
use common::convex::User;
use convex_client::leptos::{Mutation, UseMutation, UseQuery};
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_use::use_interval_fn;
use serde::Serialize;
use uuid::Uuid;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};

use self::components::sidebar::SideBar;

#[derive(Debug, Clone, Serialize)]
struct HeartBeat {
    user: String,
    #[serde(rename = "sessionId")]
    session: String,
}

impl Mutation for HeartBeat {
    type Output = ();
    fn name(&self) -> String {
        "presence:heartbeat".into()
    }
}

pub fn use_profile() -> Signal<Option<User>> {
    Signal::derive(move || {});
    if let Some(signal) = use_context() {
        signal
    } else {
        Signal::derive(move || None)
    }
}

#[component]
pub fn Servers() -> impl IntoView {
    let presence = UseMutation::new::<HeartBeat>();
    let auth = use_auth();
    let user = UseQuery::new(move || {
        auth.auth()
            .get()
            .and_then(|res| res.ok())
            .flatten()
            .map(|user| GetUser { auth: user.id })
    });
    let user = Signal::derive(move || user.get().and_then(|res| res.ok()).flatten());

    let session = RwSignal::new(Uuid::new_v4());

    #[cfg(feature = "hydrate")]
    {
        let _ = use_interval_fn(
            move || {
                if let Some(user) = user.get() {
                    presence.dispatch(HeartBeat {
                        user: user.id,
                        session: session.get().to_string(),
                    });
                }
            },
            10000,
        );
    }
    provide_context(user);
    view! {
        <SidebarProvider style="--sidebar-width: 300px">
            <SideBar/>
            <SidebarInset class="max-h-screen" >
                <Outlet/>
            </SidebarInset>
        </SidebarProvider>
    }
}
