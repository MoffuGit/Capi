use leptos::html::Div;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualItem {
    pub key: String,
    pub index: usize,
    pub start: f64,
    pub end: f64,
    pub size: f64,
}

#[derive(Clone, Copy)]
pub struct Virtualizer {
    pub virtual_items: Memo<Vec<VirtualItem>>,
    pub total_height: Signal<f64>,
    scroll_ref: NodeRef<Div>,
    all_items_data: Memo<(Vec<VirtualItem>, f64)>,
}

impl Virtualizer {
    pub fn scroll_to_key(&self, target_key: &str) {
        let (all_items, _) = self.all_items_data.get();
        if let Some(item) = all_items.iter().find(|item| item.key == target_key)
            && let Some(element) = self.scroll_ref.get()
        {
            element.set_scroll_top(item.start as i32);
        }
    }
}

pub fn use_virtualizer(
    data_size: Signal<usize>,
    scroll_ref: NodeRef<Div>,
    estimate_size: impl Fn(usize) -> f64 + Copy + Send + Sync + 'static,
    key: impl Fn(usize) -> String + Copy + Send + Sync + 'static,
    padding: usize,
) -> Virtualizer {
    let (scroll_top, set_scroll_top) = signal(0.0);
    let (client_height, set_client_height) = signal(0.0);

    #[cfg(feature = "hydrate")]
    {
        use web_sys::{AddEventListenerOptions, wasm_bindgen::prelude::Closure};

        let closure = Closure::wrap(Box::new(move || {
            if let Some(element) = scroll_ref.get_untracked() {
                set_scroll_top(element.scroll_top() as f64);
                set_client_height(element.client_height() as f64);
            }
        }) as Box<dyn Fn()>)
        .into_js_value();

        let cleanup_fn = {
            let closure_js = closure.clone();
            move || {
                use web_sys::wasm_bindgen::JsCast;

                let options = AddEventListenerOptions::new();
                options.set_passive(true);

                if let Some(element) = scroll_ref.get_untracked() {
                    let _ = element
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "scroll",
                            closure_js.as_ref().unchecked_ref(),
                            &options,
                        );
                }
            }
        };

        Effect::new(move |_| {
            if let Some(element) = scroll_ref.get() {
                use web_sys::wasm_bindgen::JsCast;

                set_scroll_top(element.scroll_top() as f64);
                set_client_height(element.client_height() as f64);

                let options = AddEventListenerOptions::new();
                options.set_passive(true);

                let _ = element.add_event_listener_with_callback_and_add_event_listener_options(
                    "scroll",
                    closure.as_ref().unchecked_ref(),
                    &options,
                );
            }

            on_cleanup({
                use send_wrapper::SendWrapper;

                let cleanup = SendWrapper::new(cleanup_fn.clone());
                move || cleanup.take()()
            });
        });
    }

    let all_items_data = Memo::new(move |_| {
        let mut items = Vec::with_capacity(data_size.get());
        let mut current_offset = 0.0;
        for i in 0..data_size.get() {
            let size = estimate_size(i);
            let item_start = current_offset;
            let item_end = current_offset + size;
            items.push(VirtualItem {
                key: key(i),
                index: i,
                start: item_start,
                end: item_end,
                size,
            });
            current_offset = item_end;
        }
        (items, current_offset)
    });

    let total_height = Signal::derive(move || all_items_data.get().1);

    let virtual_items = Memo::new(move |_| {
        let (all_items, _) = all_items_data.get();
        if all_items.is_empty() {
            return Vec::new();
        }

        let current_scroll_top = scroll_top.get();
        let current_client_height = client_height.get();
        let viewport_end = current_scroll_top + current_client_height;

        let mut start_idx = all_items.partition_point(|item| item.end < current_scroll_top);
        start_idx = start_idx.saturating_sub(padding).max(0);

        let mut end_idx = all_items.partition_point(|item| item.start < viewport_end);
        end_idx = end_idx.saturating_add(padding).min(all_items.len());

        all_items[start_idx..end_idx].to_vec()
    });

    Virtualizer {
        virtual_items,
        total_height,
        scroll_ref,
        all_items_data,
    }
}
