use capi_primitives::toasts::manager::use_toast_manager;
use capi_primitives::toasts::Toast as ToastData;
use capi_primitives::toasts::ToastPortal as ToastPortalPrimitive;
use capi_primitives::toasts::ToastProvider as ToastProviderPrimitive;
use capi_primitives::toasts::ToastRoot as ToastRootPrimitive;
use capi_primitives::toasts::ToastViewport as ToastViewportPrimitive;
use leptos::prelude::*;
use uuid::Uuid;

use super::button::Button;

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
            class="absolute bottom-4 right-4 w-[250px]"
        >
            {children()}
        </ToastViewportPrimitive>
    }
}

#[component]
pub fn ToastList() -> impl IntoView {
    let manager = use_toast_manager();
    view! {
        <For
            each=move || manager.toasts.get()
            key=|toast| toast.id
            children=move |toast| {
                view!{
                    <Toast toast=toast/>
                }
            }
        />
    }
}

#[component]
pub fn Toast(toast: ToastData) -> impl IntoView {
    view! {
        <ToastRootPrimitive class="absolute bottom-0 my-0 mx-auto w-full bg-popover data-[expanded=true]:translate-y-[calc(var(--toast-offset-y)*-1+var(--toast-index)*0.75rem*-1)] translate-y-[calc(min(var(--toast-index),10)*-20%)] data-[expanded=false]:scale-[calc(max(0,1-(var(--toast-index)*0.1)))] duration-500 transition-transform ease-out-quint after:content-[' '] after:absolute after:w-full after:left-0 after:h-[calc(0.75rem+1px)] after:top-full border border-border rounded-md p-2" toast=toast>
            {move || toast.description.get()}
        </ToastRootPrimitive>
    }
}

#[component]
pub fn ToastButton() -> impl IntoView {
    let manager = use_toast_manager();
    let (count, set_count) = signal(0);
    view! {
        <Button
            on:click=move |_| {
                manager.add.run(ToastData {
                    id: Uuid::new_v4(),
                    node_ref: NodeRef::new(),
                    title: "".into(),
                    _type: "".into(),
                    description: format!("this is a toast: {}", count.get_untracked()).into(),
                    timeout: 0,
                });
                set_count.update(|count| *count += 1);
            }
        >
            "toast"
        </Button>
    }
}
