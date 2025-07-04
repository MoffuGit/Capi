use leptos::prelude::*;

use crate::components::ui::sidebar::{
    SidebarContent, SidebarGroup, SidebarGroupContent, SidebarHeader,
};

#[component]
pub fn InboxSideBar() -> impl IntoView {
    view! {
        <SidebarHeader class="gap-3.5 border-b p-4">
            <div class="flex w-full items-center justify-between">
                <div class="text-foreground text-base font-medium">
                    "Inbox"
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
