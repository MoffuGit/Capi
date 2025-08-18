use leptos::ev::{mouseenter, mouseleave};
use leptos::prelude::*;
use leptos_use::{UseEventListenerOptions, use_event_listener_with_options};
use web_sys::{MouseEvent, Node};

use super::floating::FloatingContext;
use super::floating_tree::{use_floating_tree, use_tree_node};

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    x: f64,
    y: f64,
}

fn sign(p1: Point, p2: Point, p3: Point) -> f64 {
    (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
}

fn is_point_in_triangle(pt: Point, v1: Point, v2: Point, v3: Point) -> bool {
    let d1 = sign(pt, v1, v2);
    let d2 = sign(pt, v2, v3);
    let d3 = sign(pt, v3, v1);

    let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    !(has_neg && has_pos)
}

pub fn use_hover(
    ctx: &FloatingContext,
    open_delay: u64,
    close_delay: u64,
    enabled: Signal<bool>,
    hoverable: Signal<bool>,
) {
    let FloatingContext {
        open,
        floating_ref,
        trigger_ref,
        ..
    } = *ctx;

    let timer = StoredValue::new(None::<TimeoutHandle>);

    let (last_point, set_last_point) = signal(Point::default());
    let (current_mouse_point, set_current_mouse_point) = signal(Point::default());

    let toggle = move |entering: bool| {
        #[cfg(not(feature = "ssr"))]
        {
            let delay = if entering { open_delay } else { close_delay };

            timer.update_value(|timer| {
                if let Some(handle) = timer.take() {
                    handle.clear();
                }
            });

            if delay > 0 {
                timer.set_value(
                    set_timeout_with_handle(
                        move || open.set(entering),
                        std::time::Duration::from_millis(delay),
                    )
                    .ok(),
                );
            } else {
                open.set(entering);
            }
        }
    };

    let listener_options = UseEventListenerOptions::default().passive(true);

    let tree = use_floating_tree();
    let current_node = use_tree_node();

    let _ = use_event_listener_with_options(
        trigger_ref,
        mouseenter,
        move |_| toggle(true),
        listener_options,
    );

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::prelude::Closure;

        let handle_mouse_move = Closure::wrap(Box::new(move |evt: MouseEvent| {
            let current_point = Point {
                x: evt.client_x().into(),
                y: evt.client_y().into(),
            };
            set_current_mouse_point(current_point);
            let has_open_childs = move || {
                let mut has = false;
                if let (Some(tree_inst), Some(current_node_inst)) = (tree, current_node) {
                    let current_node_id = current_node_inst.id;
                    let has_any_open_child = tree_inst.0.with_untracked(|map| {
                        map.values().any(|node| {
                            node.parent_id == Some(current_node_id)
                                && node.context.is_some_and(|ctx| ctx.open.get_untracked())
                        })
                    });
                    has = has_any_open_child;
                }
                has
            };

            let check_hover_area = move || {
                let floating_el = floating_ref.get_untracked();
                let trigger_el = trigger_ref.get_untracked();

                if floating_el.is_none() || trigger_el.is_none() {
                    toggle(false);
                    return;
                }

                let floating_el_node = floating_el.as_ref().map(|el| el.unchecked_ref::<Node>());

                let trigger_el_node = trigger_el.as_ref().map(|el| el.unchecked_ref::<Node>());

                let event_target_node = evt.target().and_then(|t| t.dyn_into::<Node>().ok());

                let is_over_floating = if let (Some(floating_node), Some(target_node)) =
                    (floating_el_node, &event_target_node)
                {
                    floating_node.contains(Some(target_node))
                } else {
                    false
                };

                let is_over_trigger = if let (Some(trigger_node), Some(target_node)) =
                    (trigger_el_node, event_target_node)
                {
                    trigger_node.contains(Some(&target_node))
                } else {
                    false
                };

                if is_over_floating || is_over_trigger {
                    toggle(true);
                    return;
                }

                if has_open_childs() {
                    return;
                }
                let floating_rect = floating_el.unwrap().get_bounding_client_rect();

                let last_point_val = last_point.get_untracked();

                let top_dist = (last_point_val.y - floating_rect.top()).abs();
                let bottom_dist = (last_point_val.y - floating_rect.bottom()).abs();
                let left_dist = (last_point_val.x - floating_rect.left()).abs();
                let right_dist = (last_point_val.x - floating_rect.right()).abs();

                let min_dist = top_dist.min(bottom_dist).min(left_dist).min(right_dist);

                let v1 = last_point_val;

                let v2: Point;
                let v3: Point;

                if min_dist == top_dist {
                    v2 = Point {
                        x: floating_rect.left(),
                        y: floating_rect.top(),
                    };
                    v3 = Point {
                        x: floating_rect.right(),
                        y: floating_rect.top(),
                    };
                } else if min_dist == bottom_dist {
                    v2 = Point {
                        x: floating_rect.left(),
                        y: floating_rect.bottom(),
                    };
                    v3 = Point {
                        x: floating_rect.right(),
                        y: floating_rect.bottom(),
                    };
                } else if min_dist == left_dist {
                    v2 = Point {
                        x: floating_rect.left(),
                        y: floating_rect.top(),
                    };
                    v3 = Point {
                        x: floating_rect.left(),
                        y: floating_rect.bottom(),
                    };
                } else {
                    v2 = Point {
                        x: floating_rect.right(),
                        y: floating_rect.top(),
                    };
                    v3 = Point {
                        x: floating_rect.right(),
                        y: floating_rect.bottom(),
                    };
                }

                let is_over_safe_triangle = is_point_in_triangle(current_point, v1, v2, v3);

                if !is_over_safe_triangle {
                    toggle(false);
                }
            };

            check_hover_area();
        }) as Box<dyn FnMut(MouseEvent)>)
        .into_js_value();

        let closure = handle_mouse_move.clone();

        let _ = use_event_listener_with_options(
            trigger_ref,
            mouseleave,
            move |evt| {
                if !hoverable() {
                    toggle(false);
                }
                set_last_point(Point {
                    x: evt.client_x().into(),
                    y: evt.client_y().into(),
                });
                let _ = window().add_event_listener_with_callback(
                    "mousemove",
                    handle_mouse_move.as_ref().unchecked_ref(),
                );
            },
            listener_options,
        );

        let cleanup_fn = {
            let closure_js = closure.clone();
            move || {
                let _ = window().remove_event_listener_with_callback(
                    "mousemove",
                    closure_js.as_ref().unchecked_ref(),
                );
            }
        };
        on_cleanup({
            use send_wrapper::SendWrapper;

            let cleanup = SendWrapper::new(cleanup_fn);
            move || cleanup.take()()
        });

        Effect::new(move |_| {
            if !open.get() {
                let _ = window().remove_event_listener_with_callback(
                    "mousemove",
                    closure.as_ref().unchecked_ref(),
                );
            }
        });
    }
}
