#![allow(dead_code)]
use leptos::prelude::*;

pub use super::common::Orientation;

#[component]
pub fn Separator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<Orientation>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class=class data-orientation=move || orientation.get().to_string()>
            {children.map(|children| children())}
        </div>
    }
}
