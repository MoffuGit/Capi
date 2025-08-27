use leptos::ev::{animationend, transitionend};
use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::use_event_listener;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::window;

#[derive(Clone, Copy)]
pub struct AnimationFrame;

impl AnimationFrame {
    pub fn create() -> Self {
        Self {}
    }

    pub fn request(f: Rc<Closure<dyn Fn()>>) -> impl Fn() + 'static {
        let handle = window()
            .expect("should acces the window")
            .request_animation_frame((*f).as_ref().unchecked_ref())
            .unwrap();

        move || {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(handle).unwrap_or_default();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, strum_macros::EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
pub enum TransitionStatus {
    Opening,
    Closing,
    Open,
    Closed,
}

pub fn use_transition_status(
    open: Signal<bool>,
    content_node_ref: NodeRef<Div>,
) -> TransitionStatusState {
    let transition_status: RwSignal<TransitionStatus> = RwSignal::new(TransitionStatus::Closed);
    let mounted = Memo::new(move |_| {
        let open = open.get();
        let status = transition_status.get();
        open || status != TransitionStatus::Closed
    });

    #[cfg(feature = "hydrate")]
    let closure_for_animation_frame = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Opening);
    }));

    let opening = Memo::new(move |_| {
        open.get()
            && (transition_status.get() == TransitionStatus::Closed
                || transition_status.get() == TransitionStatus::Closing)
    });

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        if opening() {
            let cancel_frame = AnimationFrame::request(closure_for_animation_frame.clone());
            on_cleanup(move || {
                cancel_frame();
            });
        }
    });

    #[cfg(feature = "hydrate")]
    let ending = Rc::new(Closure::new(move || {
        transition_status.set(TransitionStatus::Closing);
    }));

    let closing = Memo::new(move |_| {
        !open.get() && mounted.get() && transition_status.get() != TransitionStatus::Closing
    });

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            if closing() {
                let cancel_frame = AnimationFrame::request(ending.clone());
                on_cleanup(move || {
                    cancel_frame();
                });
            }
        }
    });

    let idle =
        Memo::new(move |_| open.get() && transition_status.get() == TransitionStatus::Opening);

    Effect::new(move |_| {
        if idle() {
            let tr_cleanup = use_event_listener(content_node_ref, animationend, move |evt| {
                if let Some(target) = evt.target()
                    && let Ok(html_element) = target.dyn_into::<web_sys::HtmlElement>()
                    && let Some(node) = content_node_ref.get()
                    && node.is_same_node(Some(&html_element))
                {
                    transition_status.set(TransitionStatus::Open);
                }
            });

            let an_cleanup = use_event_listener(content_node_ref, transitionend, move |evt| {
                if let Some(target) = evt.target()
                    && let Ok(html_element) = target.dyn_into::<web_sys::HtmlElement>()
                    && let Some(node) = content_node_ref.get()
                    && node.is_same_node(Some(&html_element))
                {
                    transition_status.set(TransitionStatus::Open);
                }
            });

            on_cleanup(move || {
                tr_cleanup();
                an_cleanup();
            });
        }
    });

    let closed =
        Memo::new(move |_| !open.get() && transition_status.get() == TransitionStatus::Closing);

    Effect::new(move |_| {
        if closed() {
            let tr_cleanup = use_event_listener(content_node_ref, animationend, move |evt| {
                if let Some(target) = evt.target()
                    && let Ok(html_element) = target.dyn_into::<web_sys::HtmlElement>()
                    && let Some(node) = content_node_ref.get()
                    && node.is_same_node(Some(&html_element))
                {
                    transition_status.set(TransitionStatus::Closed);
                }
            });

            let an_cleanup = use_event_listener(content_node_ref, transitionend, move |evt| {
                if let Some(target) = evt.target()
                    && let Ok(html_element) = target.dyn_into::<web_sys::HtmlElement>()
                    && let Some(node) = content_node_ref.get()
                    && node.is_same_node(Some(&html_element))
                {
                    transition_status.set(TransitionStatus::Closed);
                }
            });
            on_cleanup(move || {
                tr_cleanup();
                an_cleanup();
            });
        }
    });

    TransitionStatusState {
        mounted,
        transition_status,
    }
}

#[derive(Clone, Copy)]
pub struct TransitionStatusState {
    pub mounted: Memo<bool>,
    pub transition_status: RwSignal<TransitionStatus>,
}
