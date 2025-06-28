use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::components::primitives::dialog::{DialogPopup, DialogPortal, DialogRoot, DialogTrigger};
use crate::components::ui::button::Button;

#[component]
pub fn Servers() -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <DialogRoot open=open>
            <DialogTrigger as_child=true>
                <Button >
                    "Open Dialog"
                </Button >
            </DialogTrigger>
            <DialogPortal>
                <DialogPopup/>
            </DialogPortal>
        </DialogRoot>
        <Outlet/>
    }
}
