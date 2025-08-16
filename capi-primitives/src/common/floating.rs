use std::time::Duration;

use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{UseElementBoundingReturn, use_element_bounding};
use web_sys::{MouseEvent, PointerEvent};

use super::{Align, Side};

#[derive(Debug, Clone, Copy)]
pub struct FloatingPosition {
    pub x: Memo<f64>,
    pub y: Memo<f64>,
    pub arrow: Option<UseArrow>,
}

#[derive(Debug, Clone, Copy)]
pub struct FloatingContext {
    trigger_ref: NodeRef<Div>,
    floating_ref: NodeRef<Div>,
    open: RwSignal<bool>,
    pub position_ref: RwSignal<Option<TriggerBoundingRect>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ClickHandlers {
    pub on_click: Callback<MouseEvent>,
}

pub struct UseHoverHandlers {
    pub on_pointer_enter: Callback<PointerEvent>,
    pub on_pointer_leave: Callback<PointerEvent>,
    pub close: Callback<()>,
    pub open: Callback<()>,
}

pub fn use_position(
    ctx: &FloatingContext,
    side: Signal<Side>,
    side_of_set: Signal<f64>,
    align: Signal<Align>,
    align_of_set: Signal<f64>,
    arrow: Option<UseArrowProps>,
) -> FloatingPosition {
    let FloatingContext {
        trigger_ref,
        floating_ref,
        position_ref,
        ..
    } = *ctx;
    let UseElementBoundingReturn {
        width: trigger_width,
        height: trigger_height,
        x: trigger_x,
        y: trigger_y,
        ..
    } = use_element_bounding(trigger_ref);

    let trigger_x = Memo::new(move |_| {
        if let Some(rect) = position_ref.get() {
            rect.x
        } else {
            trigger_x()
        }
    });

    let trigger_y = Memo::new(move |_| {
        if let Some(rect) = position_ref.get() {
            rect.y
        } else {
            trigger_y()
        }
    });

    let trigger_width = Memo::new(move |_| {
        if let Some(rect) = position_ref.get() {
            rect.width
        } else {
            trigger_width()
        }
    });

    let trigger_height = Memo::new(move |_| {
        if let Some(rect) = position_ref.get() {
            rect.height
        } else {
            trigger_height()
        }
    });

    let UseElementBoundingReturn {
        width: content_width,
        height: content_height,
        ..
    } = use_element_bounding(floating_ref);

    let x = Memo::new(move |_| {
        calculate_floating_x(
            trigger_x.get(),
            trigger_width.get(),
            content_width.get(),
            side(),
            align(),
            side_of_set.get(),
            align_of_set.get(),
        )
    });

    let y = Memo::new(move |_| {
        calculate_floating_y(
            trigger_y.get(),
            trigger_height.get(),
            content_height.get(),
            side(),
            align(),
            side_of_set.get(),
            align_of_set.get(),
        )
    });

    let arrow = {
        arrow.map(
            |UseArrowProps {
                 arrow_ref,
                 primary_offset,
                 secondary_offset,
             }| {
                let UseElementBoundingReturn {
                    width: arrow_width,
                    height: arrow_height,
                    ..
                } = use_element_bounding(arrow_ref);

                let x = Memo::new(move |_| {
                    arrow_x(
                        x.get(),
                        content_width.get(),
                        arrow_width.get(),
                        side(),
                        align(),
                        primary_offset.get(),
                        secondary_offset.get(),
                    )
                });

                let y = Memo::new(move |_| {
                    calculate_arrow_y(
                        y.get(),
                        content_height.get(),
                        arrow_height.get(),
                        side(),
                        align(),
                        primary_offset.get(),
                        secondary_offset.get(),
                    )
                });
                UseArrow { x, y }
            },
        )
    };

    FloatingPosition { x, y, arrow }
}

pub fn use_click(ctx: &FloatingContext) -> ClickHandlers {
    let open = ctx.open;
    let on_click = Callback::new(move |_evt| {
        open.update(|open| *open = !*open);
    });
    ClickHandlers { on_click }
}

pub fn use_hover(
    ctx: &FloatingContext,
    timeout_duration_ms: u64,
    timeout_open_duration_ms: u64,
    enabled: RwSignal<bool>,
) -> UseHoverHandlers {
    let open = ctx.open;
    let active_hovers_count = RwSignal::new(0);
    let timeout_handle = StoredValue::new(None::<TimeoutHandle>);
    let timeout_open_handle = StoredValue::new(None::<TimeoutHandle>);
    let timeout_duration_ms_sv = StoredValue::new(timeout_duration_ms);
    let timeout_open_duration_ms_sv = StoredValue::new(timeout_open_duration_ms);

    Effect::new(move |_| {
        let current_hover_count = active_hovers_count.get();
        let is_area_enabled = enabled.get();
        let is_currently_hovering_area = open.get();

        if !is_area_enabled {
            open.set(false);
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
                            open.set(true);
                        }
                        timeout_open_handle.set_value(None);
                    },
                    Duration::from_millis(timeout_open_duration_ms_sv.get_value()),
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
                            open.set(false);
                        }
                        timeout_handle.set_value(None);
                    },
                    Duration::from_millis(timeout_duration_ms_sv.get_value()),
                );
                timeout_handle.set_value(close_handle.ok());
            }
        }
    });

    let on_pointer_enter = Callback::new(move |_: PointerEvent| {
        active_hovers_count.update(|count| *count += 1);
    });

    let on_pointer_leave = Callback::new(move |_: PointerEvent| {
        active_hovers_count.update(|count| {
            if *count > 0 {
                *count -= 1;
            }
        });
    });

    let close = Callback::new(move |_| {
        open.set(false);
        active_hovers_count.set(0); // Ensure active_hovers_count is reset on explicit close
        if let Some(handle) = timeout_handle.get_value() {
            handle.clear();
            timeout_handle.set_value(None);
        }
        if let Some(handle) = timeout_open_handle.get_value() {
            handle.clear();
            timeout_open_handle.set_value(None);
        }
    });

    let open_cb = Callback::new(move |_| {
        open.set(true);
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
        open: open_cb,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerBoundingRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct UseArrow {
    pub x: Memo<f64>,
    pub y: Memo<f64>,
}

pub fn calculate_floating_x(
    trigger_x: f64,
    trigger_width: f64,
    content_width: f64,
    side: Side,
    align: Align,
    side_offset: f64,
    align_offset: f64,
) -> f64 {
    match side {
        Side::Top | Side::Bottom => match align {
            Align::Start => trigger_x + align_offset,
            Align::Center => trigger_x + (trigger_width - content_width) / 2.0 + align_offset,
            Align::End => trigger_x + trigger_width - content_width - align_offset,
        },
        Side::Left => trigger_x - content_width - side_offset,
        Side::Right => trigger_x + trigger_width + side_offset,
    }
}

pub fn calculate_floating_y(
    trigger_y: f64,
    trigger_height: f64,
    content_height: f64,
    side: Side,
    align: Align,
    side_offset: f64,
    align_offset: f64,
) -> f64 {
    match side {
        Side::Top => trigger_y - content_height - side_offset,
        Side::Bottom => trigger_y + trigger_height + side_offset,
        Side::Left | Side::Right => match align {
            Align::Start => trigger_y + align_offset,
            Align::Center => trigger_y + (trigger_height - content_height) / 2.0 + align_offset,
            Align::End => trigger_y + trigger_height - content_height - align_offset,
        },
    }
}

pub fn arrow_x(
    content_x: f64,
    content_width: f64,
    arrow_width: f64,
    side: Side,
    align: Align,
    arrow_primary_offset: f64,
    arrow_secondary_offset: f64,
) -> f64 {
    match side {
        Side::Top | Side::Bottom => match align {
            Align::Start => content_x + arrow_secondary_offset,
            Align::Center => {
                content_x + (content_width - arrow_width) / 2.0 + arrow_secondary_offset
            }
            Align::End => content_x + content_width - arrow_width - arrow_secondary_offset,
        },
        Side::Left => content_x + content_width - arrow_width - arrow_primary_offset,
        Side::Right => content_x + arrow_primary_offset,
    }
}

pub fn calculate_arrow_y(
    content_y: f64,
    content_height: f64,
    arrow_height: f64,
    side: Side,
    align: Align,
    arrow_primary_offset: f64,
    arrow_secondary_offset: f64,
) -> f64 {
    match side {
        Side::Left | Side::Right => match align {
            Align::Start => content_y + arrow_secondary_offset,
            Align::Center => {
                content_y + (content_height - arrow_height) / 2.0 + arrow_secondary_offset
            }
            Align::End => content_y + content_height - arrow_height - arrow_secondary_offset,
        },
        Side::Top => content_y + content_height - arrow_height - arrow_primary_offset,
        Side::Bottom => content_y + arrow_primary_offset,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UseArrowProps {
    arrow_ref: NodeRef<Div>,
    primary_offset: Signal<f64>,
    secondary_offset: Signal<f64>,
}

pub fn use_floating(
    trigger_ref: NodeRef<Div>,
    floating_ref: NodeRef<Div>,
    open: RwSignal<bool>,
) -> FloatingContext {
    let position_ref = RwSignal::new(None::<TriggerBoundingRect>);
    FloatingContext {
        open,
        trigger_ref,
        floating_ref,
        position_ref,
    }
}
