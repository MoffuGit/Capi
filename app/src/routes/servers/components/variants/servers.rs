use leptos::prelude::*;

use crate::components::ui::sidebar::{
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarHeader,
};

#[component]
pub fn ServersSideBar() -> impl IntoView {
    view! {
        <SidebarHeader>
            <div class="flex w-full items-center">
                <div class="text-foreground text-base font-medium">
                    "Servers"
                </div>
            </div>
        </SidebarHeader>
        <SidebarContent>
            <SidebarGroup class="px-0">
                <SidebarGroupContent>
                    <div/>
                </SidebarGroupContent>
            </SidebarGroup>
        </SidebarContent>
    }
}
