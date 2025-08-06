use leptos::context::Provider;
use leptos::prelude::*;
use web_sys::PointerEvent;

#[derive(Clone)]
pub struct HoverAreaContext {
    pub active_hovers_count: RwSignal<usize>,
    pub is_hovering_area: RwSignal<bool>,
    pub(crate) timeout_handle: StoredValue<Option<TimeoutHandle>>,
    pub enabled: RwSignal<bool>,
    pub timeout_duration_ms: StoredValue<u64>,
}

#[component]
pub fn HoverAreaProvider(
    children: Children,
    #[prop(default = 200)] timeout_duration_ms: u64,
    #[prop(into)] enabled: RwSignal<bool>,
) -> impl IntoView {
    let active_hovers_count = RwSignal::new(0);
    let is_hovering_area = RwSignal::new(false);
    let timeout_handle = StoredValue::new(None::<TimeoutHandle>);
    let timeout_duration_ms_sv = StoredValue::new(timeout_duration_ms);

    Effect::new(move |_| {
        let current_hover_count = active_hovers_count.get();
        let is_area_enabled = enabled.get();

        if is_area_enabled {
            if current_hover_count > 0 {
                is_hovering_area.set(true);
                if let Some(handle) = timeout_handle.get_value() {
                    handle.clear();
                    timeout_handle.set_value(None);
                }
            } else if timeout_handle.get_value().is_none() {
                let handle = set_timeout_with_handle(
                    move || {
                        if active_hovers_count.get_untracked() == 0 && enabled.get_untracked() {
                            is_hovering_area.set(false);
                        }
                        timeout_handle.set_value(None);
                    },
                    std::time::Duration::from_millis(timeout_duration_ms_sv.get_value()),
                );
                timeout_handle.set_value(handle.ok());
            }
        } else {
            is_hovering_area.set(false);
            if let Some(handle) = timeout_handle.get_value() {
                handle.clear();
                timeout_handle.set_value(None);
            }
        }
    });

    view! {
        <Provider
            value=HoverAreaContext {
                active_hovers_count,
                is_hovering_area,
                timeout_handle,
                enabled,
                timeout_duration_ms: timeout_duration_ms_sv,
            }
        >
            {children()}
        </Provider>
    }
}

pub fn use_hover_area_item_handlers() -> (impl Fn(PointerEvent), impl Fn(PointerEvent)) {
    let context = use_context::<HoverAreaContext>()
        .expect("`use_hover_area_item_handlers` must be used within a `HoverAreaProvider`");
    let active_hovers_count = context.active_hovers_count;

    let on_pointer_enter = move |_| {
        active_hovers_count.update(|count| *count += 1);
    };

    let on_pointer_leave = move |_| {
        active_hovers_count.update(|count| {
            if *count > 0 {
                *count -= 1;
            }
        });
    };

    (on_pointer_enter, on_pointer_leave)
}

pub fn use_is_hovering_area() -> RwSignal<bool> {
    let context = use_context::<HoverAreaContext>()
        .expect("`use_is_hovering_area` must be used within a `HoverAreaProvider`");
    context.is_hovering_area
}
