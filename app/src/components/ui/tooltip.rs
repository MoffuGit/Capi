use leptos::prelude::*;
use tailwind_fuse::tw_merge;

use crate::components::primitives::tooltip::ToolTipContent as ToolContentPrimitive;
use crate::components::primitives::tooltip::ToolTipPortal as ToolPortalPrimitive;
use crate::components::primitives::tooltip::ToolTipProvider as ToolProviderPrimitive;
use crate::components::primitives::tooltip::ToolTipSide;
use crate::components::primitives::tooltip::ToolTipTrigger as ToolTriggerPrimitive;

#[component]
pub fn ToolTipProvider(children: Children) -> impl IntoView {
    view! {
        <ToolProviderPrimitive
            {..}
            data-slot="tooltip-provider"
        >
            {children()}
        </ToolProviderPrimitive>
    }
}
#[component]
pub fn ToolTip(children: ChildrenFn) -> impl IntoView {
    view! {
        <ToolTipProvider>
            {children()}
        </ToolTipProvider>
    }
}

#[component]
pub fn ToolTipTrigger(
    children: ChildrenFn,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, default = true)] close_on_click: bool,
    // #[prop(optional)] on_click: Option<Callback<()>>,
    // #[prop(optional)] as_child: bool,
) -> impl IntoView {
    view! {
        <ToolTriggerPrimitive
            class=class
            close_on_click={close_on_click}
            {..}
            data-slot="tooltip-trigger"
        >
            {children()}
        </ToolTriggerPrimitive>
    }
}

#[component]
pub fn ToolTipContent(
    children: ChildrenFn,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, default = ToolTipSide::Right)] side: ToolTipSide,
    #[prop(optional, default = 2.0)] side_of_set: f64,
    #[prop(optional, default = false)] arrow: bool,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <ToolPortalPrimitive>
            <ToolContentPrimitive
                tooltip_side=side
                tooltip_of_side=side_of_set
                arrow=arrow
                class=Signal::derive(
                    move || tw_merge!(
                            "bg-primary border-primary border-0 text-primary-foreground ease-out-quint duration-200 animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit origin-(--radix-tooltip-content-transform-origin) rounded-md px-3 py-1.5 text-xs text-balance",
                            class.get()
                    ))
            >
                {children.get_value()()}
            </ToolContentPrimitive>
        </ToolPortalPrimitive>
    }
}
