use leptos::html::Div;
use leptos::prelude::{on_cleanup, Effect, Get, NodeRef, ReadSignal, RwSignal, Set, WriteSignal};
use leptos_dom::log;
use leptos_node_ref::AnyNodeRef;
use web_sys::MouseEvent;

#[derive(Clone)]
pub struct DialogRootContext {
    pub description_element_id: RwSignal<Option<String>>,
    pub modal: bool,
    pub set_open: WriteSignal<bool>,
    pub open: ReadSignal<bool>,
    pub title_element_id: RwSignal<Option<String>>,
    pub mounted: RwSignal<bool>,
    pub popup_ref: NodeRef<Div>,
    pub trigger_ref: NodeRef<Div>,
    pub backdrop_ref: NodeRef<Div>,
    pub internal_backdrop_ref: NodeRef<Div>,
    pub dismissible: bool,
    // floatingRootContext: FloatingRootContext;
}

pub struct DialogRouteParams {
    pub open: RwSignal<bool>,
    pub modal: bool,
    // pub on_open_change: Box<dyn Fn(bool)>,
    // pub on_open_change_complete: Box<dyn Fn(bool)>,
    pub dismissible: bool,
}

pub fn use_dialog_route(params: DialogRouteParams) -> DialogRootContext {
    let DialogRouteParams {
        open,
        modal,
        // on_open_change,
        // on_open_change_complete,
        dismissible,
    } = params;

    let (open, set_open) = open.split();

    let popup_ref = NodeRef::new();
    let trigger_ref = NodeRef::new();
    let backdrop_ref = NodeRef::new();
    let internal_backdrop_ref = NodeRef::new();

    let description_element_id = RwSignal::new(None);
    let title_element_id = RwSignal::new(None);
    let mounted = RwSignal::new(false);
    Effect::watch(
        move || (),
        move |_, _, _| {
            mounted.set(true);
        },
        true,
    );
    on_cleanup(move || {
        mounted.set(false);
    });

    DialogRootContext {
        description_element_id,
        modal,
        set_open,
        open,
        title_element_id,
        mounted,
        popup_ref,
        backdrop_ref,
        internal_backdrop_ref,
        trigger_ref,
        dismissible,
    }
}
