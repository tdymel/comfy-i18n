#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(uuid::Uuid);

impl NodeId {
    pub fn new() -> NodeId {
        NodeId::default()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        NodeId(uuid::Uuid::new_v4())
    }
}
