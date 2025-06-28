use leptos::prelude::*;

use crate::components::primitives::dialog::TransitionStatus;

// class="bg-background data-[state=open]:animate-in data-[state=undefined]:opacity-0 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border p-6 shadow-lg duration-200 sm:max-w-lg">
#[component]
pub fn DialogPopup(
    #[prop(into, optional)] class: Signal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let transition_status =
        use_context::<RwSignal<TransitionStatus>>().expect("should acces the transition context");
    view! {
        <div
            class=class
            data-state=move || {
                match transition_status.get() {
                    TransitionStatus::Starting => "open",
                    TransitionStatus::Ending => "closed",
                    TransitionStatus::Idle => "",
                    TransitionStatus::Undefined => "undefined",
                }
            }>
            {children.map(|children| children())}
        </div>
    }
}
