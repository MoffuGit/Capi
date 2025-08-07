use leptos::html::Div;
use leptos::prelude::{
    Callable, Callback, Effect, Get, NodeRef, ReadSignal, RwSignal, Set, WriteSignal, on_cleanup,
};
use leptos_dom::log;
use leptos_node_ref::AnyNodeRef;
use web_sys::MouseEvent;

use crate::common::status::{TransitionStatusState, use_transition_status};

#[derive(Clone)]
pub struct DialogRootContext {
    pub description_element_id: RwSignal<Option<String>>,
    pub modal: bool,
    pub set_open: WriteSignal<bool>,
    pub open: ReadSignal<bool>,
    pub title_element_id: RwSignal<Option<String>>,
    pub popup_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Div>,
    pub backdrop_ref: NodeRef<Div>,
    pub internal_backdrop_ref: NodeRef<Div>,
    pub dismissible: bool,
    pub transition_status: TransitionStatusState,
}

pub struct DialogRootParams {
    pub open: RwSignal<bool>,
    pub modal: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub dismissible: bool,
}

pub fn use_dialog_root(params: DialogRootParams) -> DialogRootContext {
    let DialogRootParams {
        open,
        modal,
        on_open_change,
        dismissible,
    } = params;

    let (open, set_open) = open.split();

    let popup_ref = NodeRef::new();
    let trigger_ref = NodeRef::new();
    let backdrop_ref = NodeRef::new();
    let internal_backdrop_ref = NodeRef::new();

    let description_element_id = RwSignal::new(None);
    let title_element_id = RwSignal::new(None);

    let transition_status = use_transition_status(open.into(), popup_ref, true, true);

    Effect::new(move |_| {
        if let Some(callback) = on_open_change {
            callback.run(transition_status.mounted.get());
        }
    });

    DialogRootContext {
        transition_status,
        description_element_id,
        modal,
        set_open,
        open,
        title_element_id,
        popup_ref,
        backdrop_ref,
        internal_backdrop_ref,
        trigger_ref,
        dismissible,
    }
}
