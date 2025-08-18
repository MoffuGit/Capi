use leptos::html::Div;
use leptos::prelude::{
    Callable, Callback, Effect, Get, NodeRef, ReadSignal, RwSignal, Set, StoredValue, WriteSignal,
    on_cleanup,
};
use leptos_dom::log;
use leptos_node_ref::AnyNodeRef;
use uuid::Uuid;
use web_sys::MouseEvent;

use crate::common::floating::{FloatingContext, use_floating};
use crate::common::floating_tree::use_floating_node_id;
use crate::common::status::{TransitionStatusState, use_transition_status};

#[derive(Clone)]
pub struct DialogRootContext {
    pub description_element_id: RwSignal<Option<String>>,
    pub modal: bool,
    pub open: RwSignal<bool>,
    pub title_element_id: RwSignal<Option<String>>,
    pub popup_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Div>,
    pub backdrop_ref: NodeRef<Div>,
    pub internal_backdrop_ref: NodeRef<Div>,
    pub dismissible: bool,
    pub transition_status: TransitionStatusState,
    pub floating: FloatingContext,
}

pub struct DialogRootParams {
    pub open: RwSignal<bool>,
    pub modal: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub dismissible: bool,
}

pub fn use_dialog_root(params: DialogRootParams, id: StoredValue<Uuid>) -> DialogRootContext {
    let DialogRootParams {
        open,
        modal,
        on_open_change,
        dismissible,
    } = params;

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

    let floating = use_floating(trigger_ref, popup_ref, open, Some(id));

    DialogRootContext {
        transition_status,
        description_element_id,
        modal,
        open,
        title_element_id,
        popup_ref,
        backdrop_ref,
        internal_backdrop_ref,
        trigger_ref,
        dismissible,
        floating,
    }
}
