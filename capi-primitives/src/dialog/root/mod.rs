use leptos::context::Provider;
use leptos::html::Div;
use leptos::prelude::*;

use crate::common::dismissible::{DismissibleOptions, use_dismiss};
use crate::common::floating::{FloatingContext, use_floating};
use crate::common::floating_tree::{FloatingNode, use_floating_node_id};
use crate::common::status::{TransitionStatusState, use_transition_status};

pub fn use_dialog_root_context() -> DialogRootContext {
    use_context().expect("should acces to the dialog route context")
}

#[derive(Clone)]
pub struct DialogRootContext {
    pub description_element_id: RwSignal<Option<String>>,
    pub modal: bool,
    pub open: RwSignal<bool>,
    pub title_element_id: RwSignal<Option<String>>,
    pub popup_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Div>,
    pub backdrop_ref: NodeRef<Div>,
    pub dismissible: bool,
    pub popup_status: TransitionStatusState,
    pub overlay_status: TransitionStatusState,
    pub floating: FloatingContext,
}

#[component]
pub fn DialogRoot(
    #[prop(into, default = RwSignal::new(false))] open: RwSignal<bool>,
    #[prop(default = true)] modal: bool,
    #[prop(default = true)] dismissible: bool,
    #[prop(into)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] trigger_ref: NodeRef<Div>,
    #[prop(into, optional)] popup_ref: NodeRef<Div>,
    #[prop(into, optional)] backdrop_ref: NodeRef<Div>,
    #[prop(optional)] dismiss_opts: DismissibleOptions,
    children: Children,
) -> impl IntoView {
    let id = use_floating_node_id();
    let description_element_id = RwSignal::new(None);
    let title_element_id = RwSignal::new(None);
    let popup_status = use_transition_status(open.into(), popup_ref);
    let overlay_status = use_transition_status(open.into(), backdrop_ref);

    Effect::new(move |_| {
        if let Some(callback) = on_open_change {
            callback.run(popup_status.mounted.get());
        }
    });

    let floating = use_floating(trigger_ref, popup_ref, open, Some(id));

    let context = DialogRootContext {
        popup_status,
        overlay_status,
        description_element_id,
        modal,
        open,
        title_element_id,
        popup_ref,
        backdrop_ref,
        trigger_ref,
        dismissible,
        floating,
    };
    use_dismiss(&context.floating, dismissible, dismiss_opts);
    view! {
        <FloatingNode id=id.get_value()>
            <Provider value=context>
                {children()}
            </Provider>
        </FloatingNode>
    }
}
