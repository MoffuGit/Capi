use leptos::prelude::*;
use tailwind_fuse::tw_join;
use tailwind_fuse::tw_merge;

use capi_primitives::tooltip::ToolTipAlign;
use capi_primitives::tooltip::ToolTipContent as ToolContentPrimitive;
use capi_primitives::tooltip::ToolTipPortal as ToolPortalPrimitive;
use capi_primitives::tooltip::ToolTipProvider as ToolProviderPrimitive;
use capi_primitives::tooltip::ToolTipSide;
use capi_primitives::tooltip::ToolTipTrigger as ToolTriggerPrimitive;

#[component]
pub fn ToolTip(children: ChildrenFn) -> impl IntoView {
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
    #[prop(optional, into)] side: ToolTipSide,
    #[prop(optional, into)] side_of_set: Signal<f64>,
    #[prop(optional, into)] align: Signal<ToolTipAlign>,
    #[prop(optional, into)] align_of_set: Signal<f64>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <ToolPortalPrimitive>
            <ToolContentPrimitive
                side=side
                side_of_set=side_of_set
                align=align
                align_of_set=align_of_set
                arrow=false
                class=Signal::derive(
                    move || tw_merge!(
                        "bg-primary text-primary-foreground animate-in fade-in-0 zoom-in-95 data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=closing]:zoom-out-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 z-50 w-fit origin-(--radix-tooltip-content-transform-origin) rounded-md px-3 py-1.5 text-xs text-balance duration-150",
                        class.get()
                    ))
            >
                {children.get_value()()}
            </ToolContentPrimitive>
        </ToolPortalPrimitive>
    }
}
