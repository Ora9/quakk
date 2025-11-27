use uuid::Uuid;

/// A node id, used by the [`Graph`] through structs [`VertexId`] and [`InoutId`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct NodeId {
    uuid: Uuid,
}

impl NodeId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

/// Different forms of inputs for a graph
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GraphIn {
    Text,
    Sound,
    Image,
}

/// Different forms of output for a graph
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GraphOut {
    Text,
    Sound,
    Image,
}

/// Each input or output (`inout`) in the graph have a specific id, that is
/// either inout of a node, or of the graph itself
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InoutId {
    GraphIn(GraphIn),
    GraphOut(GraphOut),
    NodeInout(NodeId, Uuid),
}

impl InoutId {
    /// Return a new random `inout` (input or output) for a node
    pub fn new_node_inout_id(node_id: NodeId) -> Self {
        Self::NodeInout(node_id, Uuid::new_v4())
    }

    /// Return `Some(NodeId)` if Self::NodeEdgepoint or None
    /// Return `Some([NodeId])` if edgepoint is attached to a node, or `None` if not
    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::NodeInout(node_id, _) => Some(*node_id),
            _ => None,
        }
    }
}

/// Id of a vertex (a conceptual )
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum VertexId {
    GraphIn,
    GraphOut,
    Node(NodeId),
}

impl VertexId {
    pub fn new_node_id() -> Self {
        Self::Node(NodeId::new())
    }

    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::Node(node_id) => Some(*node_id),
            _ => None
        }
    }
}
