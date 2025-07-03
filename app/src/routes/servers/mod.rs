mod components;

use common::user::User;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::auth::use_auth;
use crate::components::primitives::common::Orientation;
use crate::components::ui::divider::Separator;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};
use crate::sync::SyncProvider;

use self::components::sidebar::SideBar;

pub fn user_user() -> User {
    use_context().expect("should acces to the user signal")
}

#[component]
pub fn Servers() -> impl IntoView {
    let auth = use_auth().auth;
    view! {
        <Transition>
            {
                move || {
                    auth.and_then(|user| {
                        user.clone().map(|user| {
                            view!(
                                <Provider value=user>
                                    <SyncProvider>
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
                                                <Outlet/>
                                            </SidebarInset>
                                        </SidebarProvider>
                                    </SyncProvider>
                                </Provider>
                            )
                        })
                    })
                }
            }
        </Transition>
    }
}
