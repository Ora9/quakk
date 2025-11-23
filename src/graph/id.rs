use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GraphInput {
    Text,
    File,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GraphOutput {
    Terminal,
    Sound,
    Video,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum EdgepointId {
    GraphInput(GraphInput),
    GraphOutput(GraphOutput),
    NodeEdgepoint(NodeId, Uuid),
}

impl EdgepointId {
    pub fn new_node(node_id: NodeId) -> Self {
        Self::NodeEdgepoint(node_id, Uuid::new_v4())
    }

    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeEdgepoint(node_id, _) => Some(*node_id),
            _ => None,
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum NodeId {
    GraphInput,
    GraphOutput,
    Node(Uuid),
}

impl NodeId {
    pub fn new_node() -> Self {
        Self::Node(Uuid::new_v4())
    }
}
