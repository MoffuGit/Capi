use capi_primitives::separator::Orientation;
use capi_primitives::separator::Separator as SeparatorPrimitive;
use leptos::prelude::*;
use tailwind_fuse::tw_merge;

const BASE: &str = "bg-border shrink-0 data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px";

#[component]
pub fn Separator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<Orientation>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Signal::derive(move || tw_merge!(BASE, class.get()));
    view! {
        <SeparatorPrimitive class=class orientation=orientation >
            {children.map(|children| children())}
        </SeparatorPrimitive>
    }
}
