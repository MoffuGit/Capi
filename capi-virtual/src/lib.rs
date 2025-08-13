use leptos::html::Div;
use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualItem {
    pub key: usize,
    pub index: usize,
    pub start: f64,
    pub end: f64,
    pub size: f64,
}

pub struct Virtualizer {
    pub virtual_items: Memo<Vec<VirtualItem>>,
    pub total_height: Signal<f64>,
}

pub fn use_virtualizer(
    data_size: Signal<usize>,
    scroll_ref: NodeRef<Div>,
    estimate_size: f64,
    padding: usize,
) -> Virtualizer {
    let (scroll_top, set_scroll_top) = signal(0.0);
    let (client_height, set_client_height) = signal(0.0);

    #[cfg(feature = "hydrate")]
    {
        use web_sys::wasm_bindgen::prelude::Closure;

        let closure = Closure::wrap(Box::new(move || {
            if let Some(element) = scroll_ref.get() {
                set_scroll_top(element.scroll_top() as f64);
                set_client_height(element.client_height() as f64);
            }
        }) as Box<dyn Fn()>)
        .into_js_value();

        let cleanup_fn = {
            let closure_js = closure.clone();
            move || {
                use web_sys::wasm_bindgen::JsCast;

                if let Some(element) = scroll_ref.get() {
                    let _ = element.remove_event_listener_with_callback(
                        "resize",
                        closure_js.as_ref().unchecked_ref(),
                    );
                }
            }
        };

        Effect::new(move |_| {
            if let Some(element) = scroll_ref.get() {
                use web_sys::wasm_bindgen::JsCast;

                set_scroll_top(element.scroll_top() as f64);
                set_client_height(element.client_height() as f64);
                let _ = element
                    .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }

            on_cleanup({
                use send_wrapper::SendWrapper;

                let cleanup = SendWrapper::new(cleanup_fn.clone());
                move || cleanup.take()()
            });
        });
    }

    let total_height = Signal::derive(move || data_size.get() as f64 * estimate_size);

    let virtual_items = Memo::new(move |_| {
        let start_node = ((scroll_top.get() / estimate_size).floor() as usize)
            .saturating_sub(padding)
            .max(0);

        let visibles_nodes = ((client_height.get() / estimate_size).ceil() as usize
            + (2 * padding))
            .min(data_size.get() - start_node);

        (start_node..(visibles_nodes + start_node))
            .map(|idx| {
                let item_start = idx as f64 * estimate_size;
                let item_end = (idx + 1) as f64 * estimate_size;
                VirtualItem {
                    key: idx,
                    index: idx,
                    start: item_start,
                    end: item_end,
                    size: estimate_size,
                }
            })
            .collect::<Vec<_>>()
    });

    Virtualizer {
        virtual_items,
        total_height,
    }
}
