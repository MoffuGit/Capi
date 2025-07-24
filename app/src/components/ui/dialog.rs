use crate::components::primitives::dialog::DialogOverlay as DialogOverlayPrimitive;
use crate::components::primitives::dialog::DialogPopup as DialogPopupPrimitive;
use crate::components::primitives::dialog::DialogPortal as DialogPortalPrimitive;
use crate::components::primitives::dialog::DialogRoot as DialogPrimitive;
use crate::components::primitives::dialog::DialogTrigger as DialogTriggerPrimitive;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use tailwind_fuse::tw_merge;

#[component]
pub fn Dialog(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogPrimitive open=open modal=modal dismissible=dismissible>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive as_child=as_child node_ref=node_ref>
            {children.clone().map(|children| children())}
        </DialogTriggerPrimitive>
    }
}

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogPortalPrimitive open_duration=200 close_duration=190 container=container container_ref=container_ref as_child=as_child node_ref=node_ref children=children/>
    }
}

const DIALOG_POPUP: &str = "bg-background data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg duration-200 sm:max-w-lg";

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogPortal>
            <DialogOverlay/>
            <DialogPopupPrimitive as_child=as_child node_ref=node_ref
                class=Signal::derive(move || tw_merge!(DIALOG_POPUP, class.get()))>
                {children.get_value().map(|children| children())}
            </DialogPopupPrimitive>
        </DialogPortal>
    }
}

const DIALOG_OVERLAY: &str = "data-[state=open]:animate-in data-[state=undefined]:opacity-0 data-[modal=true]:cursor-pointer-none data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/50 duration-200 ease-out-cubic";

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogOverlayPrimitive as_child=as_child node_ref=node_ref class=Signal::derive(move || tw_merge!(DIALOG_OVERLAY, class.get()))>
            {children.get_value().map(|children| children())}
        </DialogOverlayPrimitive>
    }
}

#[component]
pub fn DialogHeader(children: Children) -> impl IntoView {
    view! {
        <div
            class="flex flex-col gap-2 text-center sm:text-left"
            data-slot="dialog-header"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DialogFooter(children: Children) -> impl IntoView {
    view! {
        <div
            data-slot="dialog-footer"
            class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DialogTitle(children: Children) -> impl IntoView {
    view! {
        <div class="text-lg leading-none font-semibold">
            {children()}
        </div>
    }
}

#[component]
pub fn DialogDescription(children: Children) -> impl IntoView {
    view! {
        <div class="text-muted-foreground text-sm">
            {children()}
        </div>
    }
}
