use leptos::context::Provider;
use leptos::prelude::*;

#[derive(Debug, Clone)]
pub struct CheckboxContext {
    checked: RwSignal<bool>,
    disabled: Signal<bool>,
}

#[derive(Debug, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CheckboxState {
    Checked,
    Unchecked,
}

#[component]
pub fn CheckboxRoot(
    #[prop(into, optional)] checked: RwSignal<bool>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into, default = Signal::from(false))] disabled: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <button
            role="checkbox"
            aria-checked=move || checked.get().to_string()
            on:click=move |_| {
                if !disabled.get() {
                    checked.update(|ch| {
                        *ch = !*ch
                    });
                }
            }
            disabled=disabled
            class=class
            data-state=move || {
                if checked.get() {
                    CheckboxState::Checked.to_string()
                } else {
                    CheckboxState::Unchecked.to_string()
                }
            }
        >
            <Provider value=CheckboxContext {
                checked,
                disabled
            }>
                {children.map(|children| children())}
            </Provider>
        </button>
    }
}

#[component]
pub fn CheckboxIndicator(
    #[prop(optional, into)] class: Signal<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let CheckboxContext { checked, .. } =
        use_context().expect("should access the checkbox context");
    view! {
        <Show when=move || checked.get()>
            <div
                class=class
                data-state=move || {
                    if checked.get() {
                        CheckboxState::Checked.to_string()
                    } else {
                        CheckboxState::Unchecked.to_string()
                    }
                }
            >
                {children()}
            </div>
        </Show>
    }
}
