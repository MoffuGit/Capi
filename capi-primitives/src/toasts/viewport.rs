use leptos::prelude::*;

use super::ToastContext;

#[component]
pub fn ToastViewport(
    children: Children,
    #[prop(into, optional)] class: Signal<String>,
) -> impl IntoView {
    let ToastContext {
        toasts,
        add,
        close,
        hovering,
        view_ref,
        ..
    } = use_context().expect("should acces to the toast context");

    let handle_mouse_enter = move |_| {
        hovering.set(true);
    };

    let handle_mouse_leave = move |_| {
        hovering.set(false);
    };

    let state = Memo::new(move |_| hovering.get());

    view! {
        <div
            class=class
            node_ref=view_ref
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            data-expanded=move || state.get().to_string()
        >
            {children()}
        </div>
    }
}
