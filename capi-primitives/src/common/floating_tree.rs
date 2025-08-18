use leptos::context::Provider;
use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use super::floating::FloatingContext;

#[derive(Debug, Clone, Copy)]
pub struct FloatingTreeNode {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub context: Option<FloatingContext>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FloatingTree(pub RwSignal<HashMap<Uuid, FloatingTreeNode>>);

impl FloatingTree {
    pub fn new() -> Self {
        Self(RwSignal::new(HashMap::new()))
    }

    pub fn insert_node(&self, node: FloatingTreeNode) {
        self.0.update(|map| {
            map.insert(node.id, node);
        });
    }

    pub fn remove_node(&self, id: &Uuid) {
        self.0.update(|map| {
            map.remove(id);
        });
    }

    pub fn get_node(&self, id: &Uuid) -> Option<FloatingTreeNode> {
        self.0.with(|map| map.get(id).cloned())
    }
}

pub fn provide_floating_tree() {
    provide_context(FloatingTree::new());
}

pub fn use_floating_tree() -> Option<FloatingTree> {
    use_context::<FloatingTree>()
}

pub fn provide_tree_node(node: FloatingTreeNode) {
    provide_context(node);
}

pub fn use_tree_node() -> Option<FloatingTreeNode> {
    use_context::<FloatingTreeNode>()
}

pub fn use_floating_node_id() -> StoredValue<Uuid> {
    let id = StoredValue::new(Uuid::new_v4());
    let tree = use_floating_tree();
    let parent = use_tree_node();

    Effect::new(move |_| {
        let node = FloatingTreeNode {
            id: id.get_value(),
            parent_id: parent.map(|parent| parent.id),
            context: None,
        };

        if let Some(tree) = tree {
            tree.insert_node(node);
        }
        on_cleanup(move || {
            if let Some(tree) = tree {
                tree.remove_node(&id.get_value());
            }
        });
    });
    id
}

#[component]
pub fn FloatingNode(id: Uuid, children: Children) -> impl IntoView {
    let parent = use_tree_node();
    view! {
        <Provider value=FloatingTreeNode {
            id,
            parent_id: parent.map(|parent| parent.id),
            context: None
        }>
            {children()}
        </Provider>
    }
}
