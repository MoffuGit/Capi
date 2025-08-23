pub use capi_primitives::toasts::Toast as ToastData;
use capi_primitives::toasts::ToastClose as ToastClosePrimitive;
use capi_primitives::toasts::ToastDescription as ToastDescriptionPrimitive;
use capi_primitives::toasts::ToastPortal as ToastPortalPrimitive;
use capi_primitives::toasts::ToastProvider as ToastProviderPrimitive;
use capi_primitives::toasts::ToastRoot as ToastRootPrimitive;
pub use capi_primitives::toasts::ToastStoreFields;
pub use capi_primitives::toasts::ToastStoreStoreFields;
use capi_primitives::toasts::ToastTitle as ToastTitlePrimitive;
use capi_primitives::toasts::ToastViewport as ToastViewportPrimitive;
pub use capi_primitives::toasts::manager::use_toast_store;
use icons::IconCircleCheck;
use leptos::prelude::*;
use reactive_stores::Field;

#[derive(Debug, Default, Clone, Copy, strum_macros::EnumString)]
pub enum ToastType {
    #[default]
    Message,
}

#[component]
pub fn Toasts(children: Children) -> impl IntoView {
    view! {
        <ToastProviderPrimitive>
            {children()}
            <ToastPortalPrimitive>
                <ToastView>
                    <ToastList/>
                </ToastView>
            </ToastPortalPrimitive>
        </ToastProviderPrimitive>
    }
}

#[component]
pub fn ToastView(children: Children) -> impl IntoView {
    view! {
        <ToastViewportPrimitive
            class="absolute bottom-4 right-4 w-[300px] isolate z-[100]"
        >
            {children()}
        </ToastViewportPrimitive>
    }
}

#[component]
pub fn ToastList() -> impl IntoView {
    let manager = use_toast_store();
    view! {
        <For
            each=move || manager.toasts()
            key=|toast| toast.id().get()
            children=move |toast| {
                view!{
                    <Toast toast=toast/>
                }
            }
        />
    }
}

#[component]
pub fn Toast(#[prop(into)] toast: Field<ToastData>) -> impl IntoView {
    view! {
        <ToastRootPrimitive class="absolute bottom-0 my-0 mx-auto w-full bg-popover text-popover-foreground flex items-center text-[13px] gap-2 p-3 data-[mounted=false]:opacity-0 data-[mounted=false]:translate-y-full data-[expanded=true]:translate-y-[calc(var(--toast-offset-y)*-1+var(--toast-index)*0.75rem*-1+var(--toast-swipe-movement-y))] translate-y-[calc(var(--toast-swipe-movement-y)+min(var(--toast-index),10)*-20%)] data-[expanded=false]:scale-[calc(max(0,1-(var(--toast-index)*0.1)))] duration-500 transition-all ease-out-quint after:content-[' '] after:absolute after:w-full after:left-0 after:h-[calc(0.75rem+1px)] after:top-full border border-border rounded-md data-[limited=true]:opacity-0 data-[state=closed]:opacity-0 data-[state=closed]:translate-y-full data-[state=closing]:opacity-0 data-[front=true]:data-[state=closing]:translate-y-full data-[swiping=true]:data-[state=closing]:translate-y-full data-[removed=false]:data-[swiping=true]:transition-none select-none data-[swiping=true]:before:content-[' '] data-[swiping=true]:before:absolute data-[swiping=true]:before:-left-full data-[swiping=true]:before:-right-full data-[swiping=true]:before:h-full data-[swiping=true]:before:-z-[1] data-[swiping=true]:before:bottom-1/2 data-[swiping=true]:before:scale-y-[4] data-[swiping=true]:before:translate-y-1/2" toast=toast>
            <IconCircleCheck class="size-4 text-muted-foreground"/>
            <div class="flex flex-col items-start">
                <ToastTitle/>
                <ToastDescription/>
            </div>
        </ToastRootPrimitive>
    }
}

#[component]
pub fn ToastTitle() -> impl IntoView {
    view! {
        <ToastTitlePrimitive class="text-[13px] text-popover-foreground"/>
    }
}

#[component]
pub fn ToastDescription() -> impl IntoView {
    view! {
        <ToastDescriptionPrimitive class="text-[13px] leading-5 font-light text-popover-foreground"/>
    }
}
