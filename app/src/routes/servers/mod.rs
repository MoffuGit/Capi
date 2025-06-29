use leptos::prelude::*;
use leptos_dom::log;

use crate::components::primitives::common::is_mobile;
use crate::components::ui::button::{Button, ButtonSizes};
// use crate::components::ui::dialog::{Dialog, DialogPopup, DialogTrigger};
use crate::components::ui::sheet::*;
use crate::components::ui::sidebar::{
    SideBarCollapsible, Sidebar, SidebarProvider, SidebarRail, SidebarTrigger,
};

#[component]
pub fn Servers() -> impl IntoView {
    let is_mobile = is_mobile();
    Effect::new(move |_| log!("{}", is_mobile()));
    view! {
        <SidebarProvider>
        <Sidebar collapsible=SideBarCollapsible::Icon>
            <div/>
            <SidebarRail/>
        </Sidebar>
        <SidebarTrigger/>
        </SidebarProvider>
    }
}
