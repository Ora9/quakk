use uuid::Uuid;

/// A node id, used by the [`Graph`] through structs [`VertexId`] and [`InoutId`]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct NodeId {
    id: Uuid,
}

impl NodeId {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }

    pub fn display(&self) -> String {
        self.id.to_string()
    }

    pub fn display_short(&self) -> String {
        self.id.as_fields().0.to_string()
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
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GraphInoutId {
    GraphIn(GraphIn),
    GraphOut(GraphOut),
    NodeInout(NodeId, NodeInoutId),
}

impl GraphInoutId {
    /// Return a new random `inout` (input or output) for a node
    pub fn new_node_inout_id(node_id: NodeId, inout_id: NodeInoutId) -> Self {
        Self::NodeInout(node_id, inout_id)
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

/// A node id, used by the [`Graph`] through structs [`VertexId`] and [`InoutId`]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct NodeInoutId {
    id: String,
}

impl NodeInoutId {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string()
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}


// /// Id of a vertex (a conceptual )
// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
// pub enum VertexId {
//     GraphIn,
//     GraphOut,
//     Node(NodeId),
// }

// impl VertexId {
//     pub fn new_node_id() -> Self {
//         Self::Node(NodeId::new())
//     }

//     pub fn node_id(&self) -> Option<NodeId> {
//         match self {
//             Self::Node(node_id) => Some(*node_id),
//             _ => None
//         }
//     }
// }
