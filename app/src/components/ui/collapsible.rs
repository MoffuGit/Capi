use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::primitives::collapsible::{
    CollapsiblePanel as CollapsiblePanelPrimitive, CollapsibleRoot as CollapsibleRootPrimitive,
    CollapsibleTrigger as CollapsibleTriggerPrimitive,
};

#[component]
pub fn Collapsible(
    children: Children,
    #[prop(into, optional, default = RwSignal::new(false))] open: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <CollapsibleRootPrimitive open=open>
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
pub fn CollapsiblePanel(
    children: ChildrenFn,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    view! {
        <CollapsiblePanelPrimitive class=Signal::derive(move || {
            tw_merge!(
                "overflow-hidden",
                "transition-[height,opacity]",
                "ease-out-quad",
                "duration-180",
                "data-[state=open]:opacity-100",
                "data-[state=closing]:opacity-0",
                "data-[state=closed]:opacity-0",
                "h-[var(--collapsible-panel-height)]",
                class.get()
            )
        })>
            {children()}
        </CollapsiblePanelPrimitive>
    }
}
