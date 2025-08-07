use leptos::prelude::*;

use crate::menu::MenuProviderContext;
use crate::portal::Portal;

#[component]
pub fn MenuPortal(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);
    let context: MenuProviderContext = use_context().expect("should acces to the menu context");
    let mounted = context.transition_status.mounted;
    view! {
        <Show when=move || mounted.get()>
            <Portal >
                    {children.get_value()()}
            </Portal>
        </Show>
    }
}
