use leptos::context::Provider;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

use crate::components::primitives::common::status::use_transition_status;
use crate::components::primitives::menu::MenuProviderContext;
use crate::components::primitives::portal::Portal;

#[component]
pub fn MenuPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    #[prop(default = 200)] open_duration: u64,
    #[prop(default = 200)] close_duration: u64,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context: MenuProviderContext = use_context().expect("should acces to the menu context");
    let transition_state = use_transition_status(
        context.open.read_only(),
        true,
        true,
        open_duration,
        close_duration,
    );
    let mounted = transition_state.mounted;
    let transition_status = transition_state.transition_status;
    view! {
        <Show when=move || mounted.get()>
            <Provider value=transition_state>
                <Portal >
                        {children.get_value()()}
                </Portal>
            </Provider>
        </Show>
    }
}
