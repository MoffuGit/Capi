mod backdrop;
mod content;
mod group;
mod group_label;
mod item;
mod portal;
mod separator;
mod trigger;
use leptos::context::Provider;
use leptos::{html, prelude::*};

pub use backdrop::*;
pub use content::*;
pub use group::*;
pub use group_label::*;
pub use item::*;
pub use portal::*;
pub use separator::*;
pub use trigger::*;

#[derive(Clone)]
pub struct MenuProviderContext {
    pub open: RwSignal<bool>,
    pub dismissible: bool,
    pub hidden: RwSignal<bool>,
    pub modal: bool,
    pub trigger_ref: NodeRef<html::Div>,
    pub content_ref: NodeRef<html::Div>,
}

#[component]
pub fn MenuProvider(
    children: Children,
    #[prop(optional, default = true)] modal: bool,
    #[prop(optional, into)] open: RwSignal<bool>,
    #[prop(optional, into)] hidden: RwSignal<bool>,
    #[prop(optional, into)] trigger_ref: NodeRef<html::Div>,
    #[prop(optional, into)] content_ref: NodeRef<html::Div>,
    #[prop(optional)] dismissible: bool,
) -> impl IntoView {
    view! {
        <Provider value=MenuProviderContext {
            dismissible,
            open,
            modal,
            hidden,
            trigger_ref,
            content_ref,
        }>{children()}</Provider>
    }
}
