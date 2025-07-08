mod components;
pub mod server;

use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::ui::sidebar::{SidebarInset, SidebarProvider};

use self::components::sidebar::SideBar;

#[component]
pub fn Servers() -> impl IntoView {
    view! {
        <SidebarProvider style="--sidebar-width: 300px">
            <SideBar/>
            <SidebarInset >
                <Outlet/>
            </SidebarInset>
        </SidebarProvider>
    }
}
