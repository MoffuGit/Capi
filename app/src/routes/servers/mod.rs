mod components;
pub mod server;

use api::convex::mutations::status::HeartBeat;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_use::use_interval_fn;
use uuid::Uuid;

use crate::components::auth::use_auth;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};

use self::components::sidebar::SideBar;

#[component]
pub fn Servers() -> impl IntoView {
    let presence: ServerAction<HeartBeat> = ServerAction::new();
    let auth = use_auth();

    let session = RwSignal::new(Uuid::new_v4());

    #[cfg(feature = "hydrate")]
    {
        let _ = use_interval_fn(
            move || {
                if let Some(user) = auth.user.get().flatten() {
                    presence.dispatch(HeartBeat {
                        user: user.id,
                        session: session.get().to_string(),
                    });
                }
            },
            10000,
        );
    }
    view! {
        <SidebarProvider style="--sidebar-width: 300px">
            <SideBar/>
            <SidebarInset >
                <Outlet/>
            </SidebarInset>
        </SidebarProvider>
    }
}
