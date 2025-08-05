use leptos::prelude::*;

use crate::common::Orientation;
use crate::separator::Separator;

#[component]
pub fn MenuSeparator(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(into, default = Signal::from(Orientation::Horizontal))] orientation: Signal<Orientation>,
) -> impl IntoView {
    view! {
        <Separator class=class orientation=orientation/>
    }
}
