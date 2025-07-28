mod components;
pub mod server;

use api::user::GetUser;
use common::convex::User;
use convex_client::leptos::{Mutation, UseMutation, UseQuery};
use leptos::html::option;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_location;
use leptos_use::use_interval_fn;
use serde::Serialize;
use uuid::Uuid;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};

use self::components::sidebar::{SideBar, SideBarOption};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideBarRoute {
    Server,
    Discover,
    Servers,
    Private,
}
fn get_route_from_path(path: &str) -> SideBarRoute {
    match path.split('/').nth(2) {
        None | Some("") => SideBarRoute::Servers, // Covers /servers and /servers/
        Some("discover") => SideBarRoute::Discover,
        Some("me") => SideBarRoute::Private,
        _ => SideBarRoute::Server, // Covers /servers/{id} or anything else
    }
}

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
    use_context().expect("should acces to the use profile context")
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

    let location = use_location();

    let route = Memo::new(move |_| {
        let path = location.pathname.get();
        get_route_from_path(&path)
    });

    let option: RwSignal<Option<SideBarOption>> = RwSignal::new(None);

    let shortcut = MaybeProp::derive(move || {
        if option.get().is_none()
            && (route.get() == SideBarRoute::Servers || route.get() == SideBarRoute::Discover)
        {
            None
        } else {
            Some("b".to_string())
        }
    });

    provide_context(user);
    view! {
        <SidebarProvider shortcut=shortcut style="--sidebar-width: 300px">
            <SideBar route=route option=option/>
            <SidebarInset class="max-h-screen" >
                <Outlet/>
            </SidebarInset>
        </SidebarProvider>
    }
}
