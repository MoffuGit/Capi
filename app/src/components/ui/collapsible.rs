use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::primitives::collapsible::{
    CollapsiblePanel as CollapsiblePanelPrimitive, CollapsibleRoot as CollapsibleRootPrimitive,
    CollapsibleTrigger as CollapsibleTriggerPrimitive,
};

#[component]
pub fn Collapsible(children: Children) -> impl IntoView {
    view! {
        <CollapsibleRootPrimitive open_duration=180 close_duration=180>
            {children()}
        </CollapsibleRootPrimitive>
    }
}

#[component]
pub fn CollapsibleTrigger(children: Children) -> impl IntoView {
    view! {
        <CollapsibleTriggerPrimitive>
            {children()}
        </CollapsibleTriggerPrimitive>
    }
}

#[component]
pub fn CollapsiblePanel(children: ChildrenFn) -> impl IntoView {
    view! {
        <CollapsiblePanelPrimitive class=tw_merge!(
            "overflow-hidden",
            "transition-[height,opacity]",
            "ease-out-quad",
            "duration-180",
            "data-[state=open]:opacity-100",
            "data-[state=closed]:opacity-0",
            "h-[var(--collapsible-panel-height)]",
        )>
            {children()}
        </CollapsiblePanelPrimitive>
    }
}
