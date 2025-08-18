use leptos::html::Div;
use leptos::prelude::*;
use leptos_use::{UseElementBoundingReturn, use_element_bounding};
use uuid::Uuid;
use web_sys::MouseEvent;

use super::floating_tree::use_floating_tree;
use super::{Align, Side};

#[derive(Debug, Clone, Copy)]
pub struct FloatingPosition {
    pub x: Memo<f64>,
    pub y: Memo<f64>,
    pub arrow: Option<UseArrow>,
    pub transform_origin: Memo<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct FloatingContext {
    pub trigger_ref: NodeRef<Div>,
    pub floating_ref: NodeRef<Div>,
    pub open: RwSignal<bool>,
    pub position_ref: RwSignal<Option<TriggerBoundingRect>>,
    pub id: Option<StoredValue<Uuid>>,
}

#[derive(Debug, Clone, Copy)]
pub struct ClickHandlers {
    pub on_click: Callback<MouseEvent>,
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

    let transform_origin = Memo::new(move |_| match side.get() {
        Side::Top => match align.get() {
            Align::Start => "bottom left".to_string(),
            Align::Center => "bottom center".to_string(),
            Align::End => "bottom right".to_string(),
        },
        Side::Bottom => match align.get() {
            Align::Start => "top left".to_string(),
            Align::Center => "top center".to_string(),
            Align::End => "top right".to_string(),
        },
        Side::Left => match align.get() {
            Align::Start => "right top".to_string(),
            Align::Center => "right center".to_string(),
            Align::End => "right bottom".to_string(),
        },
        Side::Right => match align.get() {
            Align::Start => "left top".to_string(),
            Align::Center => "left center".to_string(),
            Align::End => "left bottom".to_string(),
        },
    });

    FloatingPosition {
        x,
        y,
        arrow,
        transform_origin,
    }
}

pub fn use_click(ctx: &FloatingContext) -> ClickHandlers {
    let open = ctx.open;
    let on_click = Callback::new(move |_evt| {
        open.update(|open| *open = !*open);
    });
    ClickHandlers { on_click }
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
    id: Option<StoredValue<Uuid>>,
) -> FloatingContext {
    let position_ref = RwSignal::new(None::<TriggerBoundingRect>);
    let context = FloatingContext {
        open,
        trigger_ref,
        floating_ref,
        position_ref,
        id,
    };

    let tree = use_floating_tree();

    Effect::new(move |_| {
        if let (Some(tree), Some(id)) = (tree, id) {
            tree.0.update(|tree| {
                if let Some(node) = tree.get_mut(&id.get_value()) {
                    node.context = Some(context)
                }
            });
        };
    });

    context
}
