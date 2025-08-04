use leptos::context::Provider;
use leptos::prelude::*;

use crate::components::primitives::common::status::use_transition_status;
use crate::components::primitives::menu::MenuProviderContext;
use crate::components::primitives::portal::Portal;

#[component]
pub fn MenuPortal(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    let context: MenuProviderContext = use_context().expect("should acces to the menu context");
    let transition_state =
        use_transition_status(context.open.into(), context.content_ref, true, true);
    let mounted = transition_state.mounted;
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
