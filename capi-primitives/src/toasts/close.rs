use leptos::prelude::*;

use crate::toasts::ToastStoreFields;
use crate::toasts::root::ToastRootContext;

#[component]
pub fn ToastClose(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let ToastRootContext { toast, remove } =
        use_context().expect("should acces to the toast context");
    view! {
        <button class=class data-type=move || toast._type().get() on:click=move |_| {
            remove.run(())
        }>
            {children()}
        </button>
    }
}
