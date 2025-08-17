use leptos::ev::{mousemove, pointerenter, pointerleave};
use leptos::prelude::*;
use leptos_use::{
    UseElementHoverOptions, UseEventListenerOptions, use_element_bounding,
    use_element_hover_with_options, use_event_listener_with_options,
};

use super::floating::FloatingContext;

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

    let options = UseElementHoverOptions::default()
        .delay_enter(open_delay)
        .delay_leave(close_delay);

    let is_trigger_hovered = use_element_hover_with_options(trigger_ref, options);

    let (is_floating_hovered, set_is_floating_hovered) = signal(false);
    let (is_safe_polygon, set_is_safe_polygon) = signal(false);

    let is_hovered =
        Memo::new(move |_| is_trigger_hovered() || is_floating_hovered() || is_safe_polygon());

    Effect::new(move |_| {
        if enabled() && is_hovered() && hoverable() {
            let listener_options = UseEventListenerOptions::default().passive(true);
            let _ = use_event_listener_with_options(
                floating_ref,
                pointerenter,
                move |_| set_is_floating_hovered(true),
                listener_options,
            );
            let _ = use_event_listener_with_options(
                floating_ref,
                pointerleave,
                move |_| set_is_floating_hovered(false),
                listener_options,
            );
        }
    });

    Effect::new(move |_| {
        if enabled() && hoverable() {
            let floating_bounding_rect = use_element_bounding(floating_ref);
            let trigger_bounding_rect = use_element_bounding(trigger_ref);
            let listener_options = UseEventListenerOptions::default().passive(true);
            let _ = use_event_listener_with_options(
                window(),
                mousemove,
                move |evt| {
                    let mouse_x = evt.client_x() as f64;
                    let mouse_y = evt.client_y() as f64;

                    let t_x = trigger_bounding_rect.x;
                    let t_y = trigger_bounding_rect.y;
                    let t_width = trigger_bounding_rect.width;
                    let t_height = trigger_bounding_rect.height;

                    let f_x = floating_bounding_rect.x;
                    let f_y = floating_bounding_rect.y;
                    let f_width = floating_bounding_rect.width;
                    let f_height = floating_bounding_rect.height;

                    if t_width() == 0.0
                        || t_height() == 0.0
                        || f_width() == 0.0
                        || f_height() == 0.0
                    {
                        set_is_safe_polygon(false);
                        return;
                    }

                    let min_x = t_x().min(f_x());
                    let max_x = (t_x() + t_width()).max(f_x() + f_width());
                    let min_y = t_y().min(f_y());
                    let max_y = (t_y() + t_height()).max(f_y() + f_height());

                    let is_in_combined_rect = mouse_x >= min_x
                        && mouse_x <= max_x
                        && mouse_y >= min_y
                        && mouse_y <= max_y;

                    set_is_safe_polygon(is_in_combined_rect);
                },
                listener_options,
            );
        } else {
            set_is_safe_polygon(false);
        }
    });

    Effect::new(move |_| {
        if enabled() {
            open.set(is_hovered());
        }
    });
}
