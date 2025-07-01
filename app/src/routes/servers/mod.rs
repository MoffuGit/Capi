mod components;

use leptos::prelude::*;

use crate::components::primitives::common::Orientation;
use crate::components::ui::divider::Separator;
use crate::components::ui::sidebar::{SidebarInset, SidebarProvider, SidebarTrigger};

use self::components::sidebar::SideBar;

#[component]
pub fn Servers() -> impl IntoView {
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
