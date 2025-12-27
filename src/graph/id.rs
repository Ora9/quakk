//! Quack uses a few differents types of identifiers for nodes and their ins and outs :
//! - [`NodeId`] : to identify a unique node in a graph
//! - [`InoutName`] : to identify an specific "inout" (in or out of a node),
//!   but does not inlude
//! - [`NodeInoutName`] : to identify a specific "inout" of a specific node
//!
//! All of these rely on [`HashId`] which is a simple hash, either randomly determined, or based on a string
//!
//! ```text
//!      ┌─────────────┐
//!      │ NodeInoutId │
//!      └──┬───────┬──┘
//!         ▼       ▼
//!  ┌────────┐   ┌─────────┐
//!  │ NodeId │   │ InoutId │
//!  └──────┬─┘   └─┬───────┘
//!         ▼       ▼
//!      ┌────────────┐
//!      │   HashId   │
//!      └────────────┘
//! ```
//!

use std::{
    fmt::Debug,
    hash::{BuildHasher, DefaultHasher, Hasher, RandomState},
};

/// A simple hash, used for [`NodeId`], [`InoutId`]
///
/// Internaly `HashId` is an u64 hash, either a based on a string, or randomly
/// determined
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
    pub fn new_from(input: &str) -> Self {
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

/// A [`Node`](quack_sth::Node) id
/// used by the [`Graph`](quack_sth::Graph)
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

    pub fn new_node_from(input: &str) -> Self {
        Self::Node(HashId::new_from(input))
    }
}

/// Each input or output (`inout`) in the graph have a specific id, that is
/// either inout of a node, or of the graph itself
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum InoutId {
    In(HashId),
    Out(HashId),
}

impl InoutId {
    pub fn new_in_from(inout_name: &str) -> Self {
        Self::In(HashId::new_from(inout_name))
    }

    pub fn new_out_from(inout_name: &str) -> Self {
        Self::Out(HashId::new_from(inout_name))
    }

    // /// Return a new random `inout` (input or output) for a node
    // pub fn new_node_in_id(node_id: NodeId, inout_id: NodeInoutId) -> Self {
    //     Self::NodeInout(node_id, inout_id)
    // }

    // /// Return `Some(NodeId)` if Self::NodeEdgepoint or None
    // /// Return `Some([NodeId])` if edgepoint is attached to a node, or `None` if not
    // pub fn node_id(&self) -> NodeId {
    //     match self {
    //         Self::In(node_id, _) => *node_id,
    //         Self::Out(node_id, _) => *node_id,
    //     }
    // }
}

impl Debug for InoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InoutId::In(hash_id) => write!(f, "In({hash_id:?})"),
            InoutId::Out(hash_id) => write!(f, "Out({hash_id:?})"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct NodeInoutId {
    node_id: NodeId,
    inout_id: InoutId,
}

impl NodeInoutId {
    pub fn new(node_id: NodeId, inout_id: InoutId) -> Self {
        Self {
            inout_id,
            node_id,
        }
    }

    pub fn new_in_from(node_id: NodeId, inout_name: &str) -> Self {
        Self::new(node_id, InoutId::new_in_from(inout_name))
    }

    pub fn new_out_from(node_id: NodeId, inout_name: &str) -> Self {
        Self::new(node_id, InoutId::new_out_from(inout_name))
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn inout_id(&self) -> InoutId {
        self.inout_id
    }
}

impl Debug for NodeInoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}>{:?}", self.node_id(), self.inout_id())
    }
}
