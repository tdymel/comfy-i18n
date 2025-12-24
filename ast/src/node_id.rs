#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeId(uuid::Uuid);

impl NodeId {
    pub fn new() -> NodeId {
        NodeId(uuid::Uuid::new_v4())
    }
}
