use capi_primitives::common::dismissible::DismissibleOptions;
use capi_primitives::dialog::DialogOverlay as DialogOverlayPrimitive;
use capi_primitives::dialog::DialogPopup as DialogPopupPrimitive;
use capi_primitives::dialog::DialogPortal as DialogPortalPrimitive;
use capi_primitives::dialog::DialogRoot as DialogPrimitive;
use capi_primitives::dialog::DialogTrigger as DialogTriggerPrimitive;
use leptos::either::Either;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use tailwind_fuse::tw_merge;

#[component]
pub fn Dialog(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    #[prop(optional, into)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] trigger_ref: NodeRef<Div>,
    #[prop(into, optional)] popup_ref: NodeRef<Div>,
    #[prop(optional)] dismiss_opts: DismissibleOptions,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogPrimitive dismiss_opts=dismiss_opts trigger_ref=trigger_ref popup_ref=popup_ref on_open_change=on_open_change open=open modal=modal dismissible=dismissible>
            {children()}
        </DialogPrimitive>
    }
}

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <DialogTriggerPrimitive class=class>
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
        <DialogPortalPrimitive  container=container container_ref=container_ref as_child=as_child node_ref=node_ref children=children/>
    }
}

const DIALOG_POPUP: &str = "bg-background data-[state=closed]:invisible data-[state=opening]:animate-in data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 data-[state=closing]:zoom-out-95 data-[state=opening]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-2 shadow-lg duration-200 ease-out-cubic sm:max-w-lg";

#[component]
pub fn DialogPopup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional, into)] style: Signal<String>,
    #[prop(optional, into, default = Signal::derive(move || true))] overlay: Signal<bool>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogPortal>
            {
                move || {
                    if overlay() {
                        Either::Left(view!{
                            <DialogOverlay/>
                        })
                    } else {
                        Either::Right(())
                    }
                }
            }
            <DialogPopupPrimitive
                class=Signal::derive(move || tw_merge!(DIALOG_POPUP, class.get()))
                style=style
            >
                {children.get_value().map(|children| children())}
            </DialogPopupPrimitive>
        </DialogPortal>
    }
}

const DIALOG_OVERLAY: &str = "data-[state=opening]:animate-in data-[state=closed]:invisible data-[modal=true]:cursor-pointer-none data-[state=closing]:animate-out data-[state=closing]:fade-out-0 data-[state=opening]:fade-in-0 fixed inset-0 z-50 bg-black/50 ease-out-cubic duration-200";

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <DialogOverlayPrimitive class=Signal::derive(move || tw_merge!(DIALOG_OVERLAY, class.get()))>
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
