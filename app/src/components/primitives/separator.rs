#![allow(dead_code)]
use leptos::prelude::*;

pub use super::common::Orientation;

#[component]
pub fn Separator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<
        crate::components::primitives::separator::Orientation,
    >,
) -> impl IntoView {
    view! {
        <div class=class data-orientation=move || orientation.get().to_string()/>
    }
}
