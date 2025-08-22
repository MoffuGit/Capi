use leptos::prelude::*;

use crate::portal::Portal;

#[component]
pub fn ToastPortal(children: ChildrenFn) -> impl IntoView {
    view! {
        <Portal>
            {children()}
        </Portal>
    }
}
