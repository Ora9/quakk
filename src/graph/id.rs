use std::{
    fmt::Debug,
    hash::{BuildHasher, DefaultHasher, Hasher, RandomState},
};


/// A unique id, used for [`NodeId`], [`InoutId`]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct HashId {
    id: u64,
}

impl HashId {
    /// Get a new random unique id
    /// ```
    /// # use quack::HashId;
    /// assert_ne!(HashId::new(), HashId::new());
    /// ```
    pub fn new() -> Self {
        Self {
            id: RandomState::new().build_hasher().finish(),
        }
    }

    /// Get a new unique id based on a string input
    /// ```
    /// # use quack::HashId;
    /// assert_eq!(HashId::new_with("test"), HashId::new_with("test"));
    /// assert_ne!(HashId::new_with("test"), HashId::new_with("other"));
    /// ```
    pub fn new_with(input: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        hasher.write(input.as_bytes());
        hasher.write_u8(0xff);

        Self {
            id: hasher.finish(),
        }
    }

    pub fn display(&self) -> String {
        format!("{:x}", self.id)
    }

    pub fn display_short(&self) -> String {
        let mut out = Self::display(self);
        out.truncate(out.floor_char_boundary(12));
        out
    }
}

impl Debug for HashId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.id)
    }
}

impl Default for HashId {
    fn default() -> Self {
        Self::new()
    }
}

/// A node id, used by the [`Graph`] through structs [`VertexId`] and [`InoutId`]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum NodeId {
    GraphIn,
    GraphOut,
    Node(HashId),
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeId::GraphIn => write!(f, "GraphIn"),
            NodeId::GraphOut => write!(f, "GraphOut"),
            NodeId::Node(hash_id) => write!(f, "Node({hash_id:?})"),
        }
    }
}

impl NodeId {
    pub fn new_node() -> Self {
        Self::Node(HashId::new())
    }

    pub fn new_node_with(input: &str) -> Self {
        Self::Node(HashId::new_with(input))
    }
}

/// Each input or output (`inout`) in the graph have a specific id, that is
/// either inout of a node, or of the graph itself
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum InoutId {
    In(NodeId, HashId),
    Out(NodeId, HashId),
}

impl InoutId {
    // /// Return a new random `inout` (input or output) for a node
    // pub fn new_node_in_id(node_id: NodeId, inout_id: NodeInoutId) -> Self {
    //     Self::NodeInout(node_id, inout_id)
    // }

    /// Return `Some(NodeId)` if Self::NodeEdgepoint or None
    /// Return `Some([NodeId])` if edgepoint is attached to a node, or `None` if not
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::In(node_id, _) => *node_id,
            Self::Out(node_id, _) => *node_id,
        }
    }
}

impl Debug for InoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InoutId::In(node_id, hash_id) => write!(f, "In({node_id:?}>{hash_id:?}"),
            InoutId::Out(node_id, hash_id) => write!(f, "Out({node_id:?}>{hash_id:?}"),
        }
    }
}
