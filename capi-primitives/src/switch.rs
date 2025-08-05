use leptos::context::Provider;
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct SwitchContext {
    checked: RwSignal<bool>,
    disabled: Signal<bool>,
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum SwitchState {
    Checked,
    Unchecked,
}

#[component]
pub fn SwitchRoot(
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <button
            on:click=move |_| {
                checked.update(|ch| {
                    *ch = !*ch
                });
            }
            disabled=disabled
            class=class
            data-state=move || {
                if checked.get() {
                    SwitchState::Checked.to_string()
                } else {
                    SwitchState::Unchecked.to_string()
                }
            }
        >
            <Provider value=SwitchContext {
                checked,
                disabled
            }>
                {children.map(|children| children())}
            </Provider>
        </button>
    }
}

#[component]
pub fn SwitchTumb(#[prop(optional, into)] class: Signal<String>) -> impl IntoView {
    let SwitchContext { checked, .. } = use_context().expect("should acces to the switch context");
    view! {
        <span
            class=class
            data-state=move || {
                if checked.get() {
                    SwitchState::Checked.to_string()
                } else {
                    SwitchState::Unchecked.to_string()
                }
            }
        />
    }
}
