use leptos::context::Provider;
use leptos::prelude::*;
use web_sys::PointerEvent;

#[derive(Clone, Copy)]
pub struct HoverAreaContext {
    pub active_hovers_count: RwSignal<usize>,
    pub is_hovering: RwSignal<bool>,
    pub(crate) timeout_handle: StoredValue<Option<TimeoutHandle>>,
    pub(crate) timeout_open_handle: StoredValue<Option<TimeoutHandle>>,
    pub enabled: RwSignal<bool>,
    pub timeout_duration_ms: StoredValue<u64>,
    pub timeout_open_duration_ms: StoredValue<u64>,
    pub prev_context: StoredValue<Option<HoverAreaContext>>,
}

#[component]
pub fn HoverAreaProvider(
    children: Children,
    #[prop(default = 200)] timeout_duration_ms: u64,
    #[prop(default = 0)] timeout_open_duration_ms: u64,
    #[prop(into)] enabled: RwSignal<bool>,
    #[prop(into, optional)] is_hovering: RwSignal<bool>,
) -> impl IntoView {
    let parent_hover_context = StoredValue::new(use_context::<HoverAreaContext>());

    let active_hovers_count = RwSignal::new(0);
    let timeout_handle = StoredValue::new(None::<TimeoutHandle>);
    let timeout_open_handle = StoredValue::new(None::<TimeoutHandle>);
    let timeout_duration_ms_sv = StoredValue::new(timeout_duration_ms);
    let timeout_open_duration_ms_sv = StoredValue::new(timeout_open_duration_ms);

    Effect::new(move |_| {
        let current_hover_count = active_hovers_count.get();
        let is_area_enabled = enabled.get();
        let is_currently_hovering_area = is_hovering.get();

        if !is_area_enabled {
            is_hovering.set(false);
            if let Some(handle) = timeout_handle.get_value() {
                handle.clear();
                timeout_handle.set_value(None);
            }
            if let Some(handle) = timeout_open_handle.get_value() {
                handle.clear();
                timeout_open_handle.set_value(None);
            }
            return;
        }

        if current_hover_count > 0 {
            if let Some(handle) = timeout_handle.get_value() {
                handle.clear();
                timeout_handle.set_value(None);
            }

            if !is_currently_hovering_area && timeout_open_handle.get_value().is_none() {
                let open_handle = set_timeout_with_handle(
                    move || {
                        if active_hovers_count.get_untracked() > 0 && enabled.get_untracked() {
                            is_hovering.set(true);
                        }
                        timeout_open_handle.set_value(None);
                    },
                    std::time::Duration::from_millis(timeout_open_duration_ms_sv.get_value()),
                );
                timeout_open_handle.set_value(open_handle.ok());
            }
        } else {
            if let Some(handle) = timeout_open_handle.get_value() {
                handle.clear();
                timeout_open_handle.set_value(None);
            }

            if is_currently_hovering_area && timeout_handle.get_value().is_none() {
                let close_handle = set_timeout_with_handle(
                    move || {
                        if active_hovers_count.get_untracked() == 0 && enabled.get_untracked() {
                            is_hovering.set(false);
                        }
                        timeout_handle.set_value(None);
                    },
                    std::time::Duration::from_millis(timeout_duration_ms_sv.get_value()),
                );
                timeout_handle.set_value(close_handle.ok());
            }
        }
    });

    view! {
        <Provider
            value=HoverAreaContext {
                active_hovers_count,
                is_hovering,
                timeout_handle,
                timeout_open_handle,
                enabled,
                timeout_duration_ms: timeout_duration_ms_sv,
                timeout_open_duration_ms: timeout_open_duration_ms_sv,
                prev_context: parent_hover_context,
            }
        >
            {children()}
        </Provider>
    }
}

pub struct UseHoverHandlers {
    pub on_pointer_enter: Callback<PointerEvent>,
    pub on_pointer_leave: Callback<PointerEvent>,
    pub close: Callback<()>,
    pub open: Callback<()>,
}

pub fn use_hover_area_item_handlers() -> UseHoverHandlers {
    let context = use_context::<HoverAreaContext>().expect("should acces to the use hover context");

    let active_hovers_count = context.active_hovers_count;
    let is_hovering_area = context.is_hovering;
    let timeout_handle = context.timeout_handle;
    let timeout_open_handle = context.timeout_open_handle;
    let prev_context = context.prev_context;

    let on_pointer_enter = Callback::new(move |_: PointerEvent| {
        active_hovers_count.update(|count| *count += 1);
        if let Some(prev_ctx) = prev_context.get_value() {
            prev_ctx.active_hovers_count.update(|count| *count += 1);
        }
    });

    let on_pointer_leave = Callback::new(move |_: PointerEvent| {
        active_hovers_count.update(|count| {
            if *count > 0 {
                *count -= 1;
            }
        });
        if let Some(prev_ctx) = prev_context.get_value() {
            prev_ctx.active_hovers_count.update(|count| {
                if *count > 0 {
                    *count -= 1;
                }
            });
        }
    });

    let close = Callback::new(move |_| {
        is_hovering_area.set(false);
        active_hovers_count.set(0);
        if let Some(handle) = timeout_handle.get_value() {
            handle.clear();
            timeout_handle.set_value(None);
        }
        if let Some(handle) = timeout_open_handle.get_value() {
            handle.clear();
            timeout_open_handle.set_value(None);
        }
    });

    let open = Callback::new(move |_| {
        is_hovering_area.set(true);
        if let Some(handle) = timeout_handle.get_value() {
            handle.clear();
            timeout_handle.set_value(None);
        }
        if let Some(handle) = timeout_open_handle.get_value() {
            handle.clear();
            timeout_open_handle.set_value(None);
        }
    });

    UseHoverHandlers {
        on_pointer_enter,
        on_pointer_leave,
        close,
        open,
    }
}
