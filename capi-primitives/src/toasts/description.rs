use leptos::prelude::*;

use crate::toasts::ToastStoreFields;
use crate::toasts::root::ToastRootContext;

#[component]
pub fn ToastDescription(#[prop(into, optional)] class: Signal<String>) -> impl IntoView {
    let ToastRootContext { toast, .. } = use_context().expect("should acces to the toast context");
    view! {
        <div class=class data-type=move || toast._type().get()>
            {move || toast.description().get()}
        </div>
    }
}
