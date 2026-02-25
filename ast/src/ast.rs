use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    CompositeValue,
    identifier::{Identifier, Path},
    node_id::NodeId,
    node_value::NodeValue,
    spanned_ast::SpannedAst,
};

#[derive(Debug, Clone)]
pub struct Ast {
    pub identifier: Identifier,
    pub value: NodeValue<Self>,
    pub id: NodeId,
    pub parent: Option<NodeId>,
}

impl<Span> From<SpannedAst<Span>> for Ast {
    fn from(spanned_ast: SpannedAst<Span>) -> Self {
        let value = match spanned_ast.value {
            NodeValue::Composite { children, value } => NodeValue::Composite {
                children: children
                    .into_iter()
                    .map(|(key, value)| (key, Ast::from(value)))
                    .collect(),
                value,
            },
            NodeValue::Literal(literal) => NodeValue::Literal(literal),
        };

        Self {
            identifier: spanned_ast.identifier,
            id: spanned_ast.id,
            parent: spanned_ast.parent,
            value,
        }
    }
}

impl Ast {
    pub fn new(identifier: Identifier, mut value: NodeValue<Self>) -> Self {
        let id = NodeId::new();
        if let NodeValue::Composite { children, .. } = &mut value {
            children
                .iter_mut()
                .for_each(|child| child.1.attach_to_parent(id));
        }

        let result = Self {
            identifier,
            value,
            parent: None,
            id,
        };

        // Better safe than sorry
        assert!(
            result.traverse().map(|node| node.id).all_unique(),
            "All node Ids should be unique!"
        );

        result
    }

    pub fn attach_to_parent(&mut self, parent_id: NodeId) {
        self.parent = Some(parent_id);
    }

    pub fn detach_from_parent(&mut self) {
        self.parent = None;
    }

    /// Merges two nodes by following rules
    /// 1. If A does not contain subtree B, add B as a child
    /// 1. Otherwise traverse B
    pub fn merge(&mut self, other: Self) {
        if let Some(children) = other.children_into() {
            children.for_each(|other_child| {
                if let Some(existing_child) = self.get_mut(&other_child.identifier) {
                    existing_child.merge(other_child);
                } else {
                    self.add_child(other_child);
                }
            })
        }
    }

    pub fn add_child(&mut self, mut node: Self) {
        match &mut self.value {
            NodeValue::Composite { children, value } => {
                match value {
                    CompositeValue::Struct => {
                        assert!(matches!(node.identifier, Identifier::Field(..)))
                    }
                    _ => {
                        assert!(matches!(node.identifier, Identifier::TupleIndex(..)))
                    }
                }

                node.attach_to_parent(self.id);
                children.insert(node.identifier.clone(), node);
            }
            _ => panic!("comfy-i18n-ast: Can only add child to composite!"),
        }
    }

    pub fn get_mut(&mut self, identifier: &Identifier) -> Option<&mut Self> {
        match &mut self.value {
            NodeValue::Composite { children, .. } => children.get_mut(identifier),
            _ => None,
        }
    }

    pub fn get(&self, identifier: &Identifier) -> Option<&Self> {
        match &self.value {
            NodeValue::Composite { children, .. } => children.get(identifier),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<impl Iterator<Item = &Self>> {
        match &self.value {
            NodeValue::Composite { children, .. } => Some(children.values()),
            _ => None,
        }
    }

    pub fn children_into(self) -> Option<impl Iterator<Item = Self>> {
        match self.value {
            NodeValue::Composite { children, .. } => Some(children.into_values()),
            _ => None,
        }
    }

    pub fn traverse(&self) -> impl Iterator<Item = &Self> {
        let mut stack = vec![self];
        std::iter::from_fn(move || {
            let current = stack.pop()?;
            if let Some(children) = current.children() {
                stack.extend(children);
            }
            Some(current)
        })
    }

    pub fn by_path(&self, path: &Path) -> Option<&Self> {
        path.iter().try_fold(self, |current_node, identifier| {
            if let NodeValue::Composite { children, .. } = &current_node.value {
                children.get(identifier)
            } else {
                None
            }
        })
    }

    pub fn path(&self, tree: &HashMap<NodeId, &Self>) -> Path {
        if let Some(parent_id) = self.parent {
            let parent_path = tree.get(&parent_id).unwrap().path(tree);
            parent_path.append(self.identifier.clone())
        } else {
            Path::from(self.identifier.clone())
        }
    }

    pub fn id_to_node_map(&self) -> HashMap<NodeId, &Self> {
        self.traverse().map(|node| (node.id, node)).collect()
    }

    pub fn id_to_path_map(&self) -> HashMap<NodeId, Path> {
        let tree = self.id_to_node_map();
        self.traverse()
            .map(|node| (node.id, node.path(&tree)))
            .collect()
    }

    pub fn path_to_node_id_map(&self) -> HashMap<Path, NodeId> {
        let tree = self.id_to_node_map();
        self.traverse()
            .map(|node| (node.path(&tree), node.id))
            .collect()
    }

    pub fn path_to_node_map(&self) -> HashMap<Path, &Self> {
        let tree = self.id_to_node_map();
        self.traverse()
            .map(|node| (node.path(&tree), node))
            .collect()
    }
}
