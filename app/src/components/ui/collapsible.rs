use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::primitives::collapsible::{
    CollapsiblePanel as CollapsiblePanelPrimitive, CollapsibleRoot as CollapsibleRootPrimitive,
    CollapsibleTrigger as CollapsibleTriggerPrimitive,
};

#[component]
pub fn Collapsible(children: Children) -> impl IntoView {
    view! {
        <CollapsibleRootPrimitive>
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
            "transition-[height]", // Animate the height property
            "ease-out-quad",        // Easing function for the transition
            "duration-150",       // Duration of the transition (e.g., 300ms)
            "h-[var(--collapsible-panel-height)]", // Expand to measured height when open
        )>
            {children()}
        </CollapsiblePanelPrimitive>
    }
}
