use itertools::Itertools;

use crate::{Identifier, NodeId, NodeValue};

#[derive(Debug, Clone)]
pub struct SpannedAst<Span> {
    pub identifier: Identifier,
    pub value: NodeValue<Self>,
    pub span: Span,
    pub id: NodeId,
    pub parent: Option<NodeId>,
}

impl<Span> SpannedAst<Span> {
    pub fn new(identifier: Identifier, span: Span, mut value: NodeValue<Self>) -> Self {
        let id = NodeId::new();
        if let NodeValue::Composite { children, .. } = &mut value {
            children
                .iter_mut()
                .for_each(|child| child.1.attach_to_parent(id));
        }

        let result = Self {
            identifier,
            value,
            span,
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

    pub fn children(&self) -> Option<impl Iterator<Item = &Self>> {
        match &self.value {
            NodeValue::Composite { children, .. } => Some(children.values()),
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
}
