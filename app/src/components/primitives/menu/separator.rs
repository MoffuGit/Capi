use crate::components::primitives::separator::Orientation;
use leptos::prelude::*;

use crate::components::primitives::separator::Separator;

#[component]
pub fn MenuSeparator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<
        crate::components::primitives::separator::Orientation,
    >,
) -> impl IntoView {
    view! {
        <Separator class=class orientation=orientation/>
    }
}
