pub mod use_dialog_root;

use leptos::context::Provider;
use leptos::prelude::*;

use crate::common::dismissible::use_dismiss;
use crate::common::floating::use_floating;
use crate::common::floating_tree::{FloatingNode, use_floating_node_id};

use self::use_dialog_root::{DialogRootContext, DialogRootParams, use_dialog_root};

pub fn use_dialog_root_context() -> DialogRootContext {
    use_context().expect("should acces to the dialog route context")
}

#[component]
pub fn DialogRoot(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    #[prop(into)] on_open_change: Option<Callback<bool>>,
    children: Children,
) -> impl IntoView {
    let id = use_floating_node_id();
    let context = use_dialog_root(
        DialogRootParams {
            on_open_change,
            open,
            modal,
            dismissible,
        },
        id,
    );
    use_dismiss(&context.floating, dismissible);
    view! {
        <FloatingNode id=id.get_value()>
            <Provider value=context>
                {children()}
            </Provider>
        </FloatingNode>
    }
}
