use leptos::{context::Provider, prelude::*};

#[derive(Clone)]
pub struct MenuRadioGroupContext {
    pub value: RwSignal<String>,
}

#[derive(Clone)]
pub struct MenuRadioItemContext {
    pub item_value: Signal<String>,
}

#[component]
pub fn MenuRadioGroupProvider(
    children: Children,
    #[prop(into)] value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <Provider value=MenuRadioGroupContext { value }>
            {children()}
        </Provider>
    }
}

#[component]
pub fn MenuRadioGroupItem(
    children: Children,
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] class: Signal<String>,
) -> impl IntoView {
    let radio_group_context = use_context::<MenuRadioGroupContext>()
        .expect("MenuRadioGroupItem must be used within a MenuRadioGroupProvider");

    view! {
        <Provider value=MenuRadioItemContext { item_value: value }>
            <div
                class=class
                on:click=move |_| {
                    radio_group_context.value.set(value.get());
                }
            >
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn MenuRadioGroupItemIndicator(children: ChildrenFn) -> impl IntoView {
    let radio_group_context = use_context::<MenuRadioGroupContext>()
        .expect("MenuRadioGroupItemIndicator must be used within a MenuRadioGroupProvider");
    let radio_item_context = use_context::<MenuRadioItemContext>()
        .expect("MenuRadioGroupItemIndicator must be used within a MenuRadioGroupItem");

    let is_selected =
        Memo::new(move |_| radio_group_context.value.get() == radio_item_context.item_value.get());

    view! {
        <Show when=is_selected>
            {children()}
        </Show>
    }
}
