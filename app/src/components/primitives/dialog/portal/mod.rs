use crate::components::primitives::common::status::use_transition_status;
use std::sync::Arc;

use leptos::context::Provider;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::components::primitives::portal::Portal;

use super::root::use_dialog_root_context;

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(default = 200)] open_duration: u64,
    #[prop(default = 200)] close_duration: u64,
    children: StoredValue<Arc<dyn Fn() -> AnyView + Send + Sync + 'static>>,
) -> impl IntoView {
    let context = use_dialog_root_context();
    let transition_state =
        use_transition_status(context.open, true, true, open_duration, close_duration);
    let mounted = transition_state.mounted;
    let transition_status = transition_state.transition_status;
    view! {
        <Show when=move || mounted.get()>
            <Provider value=transition_status>
                <Portal container=container container_ref=container_ref as_child=as_child node_ref=node_ref>
                        {children.get_value()()}
                </Portal>
            </Provider>
        </Show>
    }
}
