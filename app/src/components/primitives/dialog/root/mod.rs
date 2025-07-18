pub mod use_dialog_root;

use leptos::context::Provider;
use leptos::prelude::*;

use self::use_dialog_root::{use_dialog_root, DialogRootContext, DialogRootParams};

pub fn use_dialog_root_context() -> DialogRootContext {
    use_context().expect("should acces to the dialog route context")
}

#[component]
pub fn DialogRoot(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    children: Children,
) -> impl IntoView {
    let dialog_root = use_dialog_root(DialogRootParams {
        open,
        modal,
        dismissible,
    });
    view! {
        <Provider value=dialog_root>
            {children()}
        </Provider>
    }
}
