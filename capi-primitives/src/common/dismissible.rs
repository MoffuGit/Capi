use leptos::ev;
use leptos::prelude::*;
use leptos_use::{UseEventListenerOptions, use_event_listener_with_options};
use uuid::Uuid;
use web_sys::Node;

use super::floating::FloatingContext;
use super::floating_tree::FloatingTreeNode;
use super::floating_tree::use_floating_tree;
use crate::common::floating_tree::use_tree_node;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy)]
pub struct DismissibleOptions {
    pub escape_key: bool,
    pub outside_press: bool,
}

impl Default for DismissibleOptions {
    fn default() -> Self {
        Self {
            escape_key: true,
            outside_press: true,
        }
    }
}

pub fn use_dismiss(ctx: &FloatingContext, enabled: bool, options: DismissibleOptions) {
    let FloatingContext {
        open, floating_ref, ..
    } = *ctx;

    let tree = use_floating_tree();
    let current_node = use_tree_node();
    Effect::new(move |_| {
        if open.get() {
            if options.outside_press {
                let _ = use_event_listener_with_options(
                    window(),
                    ev::pointerdown,
                    move |evt| {
                        if !enabled {
                            return;
                        }

                        let event_target_node = evt
                            .target()
                            .and_then(|t| t.dyn_into::<Node>().ok())
                            .expect("On what cases this should fail?");

                        let floating_el = floating_ref.get_untracked();

                        let is_click_on_self = floating_el
                            .as_ref()
                            .is_some_and(|el| el.contains(Some(&event_target_node)));

                        if is_click_on_self {
                            return;
                        }

                        let is_click_on_another_open_tree_node =
                            if let (Some(tree_inst), Some(current_node)) = (tree, current_node) {
                                tree_inst.0.with_untracked(|map| {
                                    let current_node_id_clone = current_node.id;

                                    let is_descendant_of_ancestor =
                                        |candidate_node_id: &Uuid,
                                         ancestor_id: &Uuid,
                                         all_nodes: &std::collections::HashMap<
                                            Uuid,
                                            FloatingTreeNode,
                                        >| {
                                            let mut current_id_to_check = candidate_node_id;
                                            while let Some(node_in_chain) =
                                                all_nodes.get(current_id_to_check)
                                            {
                                                if let Some(parent_id) = &node_in_chain.parent_id {
                                                    if parent_id == ancestor_id {
                                                        return true;
                                                    }
                                                    current_id_to_check = parent_id;
                                                } else {
                                                    break;
                                                }
                                            }
                                            false
                                        };

                                    map.values().any(|node| {
                                        let is_relevant_descendant = node.parent_id
                                            == Some(current_node_id_clone)
                                            || is_descendant_of_ancestor(
                                                &node.id,
                                                &current_node_id_clone,
                                                map,
                                            );

                                        is_relevant_descendant
                                            && node.context.is_some_and(|context| {
                                                context.floating_ref.get_untracked().is_some_and(
                                                    |floating_ref| {
                                                        &event_target::<web_sys::EventTarget>(&evt)
                                                            == floating_ref.as_ref()
                                                            || evt
                                                                .composed_path()
                                                                .includes(floating_ref.as_ref(), 0)
                                                    },
                                                )
                                            })
                                    })
                                })
                            } else {
                                false
                            };

                        if is_click_on_another_open_tree_node {
                            return;
                        }

                        open.set(false);
                    },
                    UseEventListenerOptions::default().passive(true),
                );
            }

            if options.escape_key {
                let _ = use_event_listener_with_options(
                    window(),
                    ev::keydown,
                    move |evt| {
                        if !enabled {
                            return;
                        }
                        if evt.key() == "Escape" {
                            open.set(false);
                        }
                    },
                    UseEventListenerOptions::default().passive(true),
                );
            }
        }
    });
}
