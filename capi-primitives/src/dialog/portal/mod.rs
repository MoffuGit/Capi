use std::sync::Arc;

use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::common::status::use_transition_status;
use crate::portal::Portal;

use super::root::use_dialog_root_context;

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: StoredValue<Arc<dyn Fn() -> AnyView + Send + Sync + 'static>>,
) -> impl IntoView {
    let context = use_dialog_root_context();
    let popup_mounted = context.popup_status.mounted;
    let backdrop_mounted = context.overlay_status.mounted;
    let mounted = Memo::new(move |_| popup_mounted() || backdrop_mounted());
    view! {
        <Show when=move || mounted.get()>
            <Portal container=container container_ref=container_ref as_child=as_child node_ref=node_ref>
                    {children.get_value()()}
            </Portal>
        </Show>
    }
}
