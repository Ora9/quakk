//! Quakk uses a few differents types of identifiers for nodes and their ins and outs :
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
    /// # use quakk::HashId;
    /// assert_ne!(HashId::new(), HashId::new());
    /// ```
    pub fn new() -> Self {
        Self {
            id: RandomState::new().build_hasher().finish(),
        }
    }

    /// Get a new unique id based on a string input
    /// ```
    /// # use quakk::HashId;
    /// assert_eq!(HashId::new_from("test"), HashId::new_from("test"));
    /// assert_ne!(HashId::new_from("test"), HashId::new_from("other"));
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

/// A [`Node`](quakk::Node) id used to identify a node
///
/// It allows representing `GraphIn` and `GraphOut`. Thoses are specials types of [`Node`](quakk::Node)
/// that handle ins and outs for the [`Graph`](quakk::Graph), theses can only exists once of each
/// in a graph, so they have this special `NodeId` representation
///
/// Other conventional nodes are identified with an [`HashId`], usually random
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum NodeId {
    GraphIn,
    GraphOut,
    Node(HashId),
}

impl NodeId {
    /// Return a new random [`NodeId`]
    pub fn new_node() -> Self {
        Self::Node(HashId::new())
    }

    /// Return a new [`NodeId`] based on an input `&str`
    pub fn new_node_from(input: &str) -> Self {
        Self::Node(HashId::new_from(input))
    }

    /// Return a [`NodeInoutId`] based on self and the given [`InoutId`]
    pub fn into_node_inout_id(self, inout_id: InoutId) -> NodeInoutId {
        NodeInoutId::new(self, inout_id)
    }
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

/// In the [`Graph`](quakk::Graph), each of the [`Node`s](quakk::Node) ins or outs have an id.
///
/// This id is designed to be unique for a specific node, but not to be unique in the graph, This id
/// only care about the inout without specifing the node it is tied to, that would be the purpose
/// of [`NodeInoutId`], that identify a specific inout in the graph.
///
/// The term `inout` is widely used in the code and documentation to refer to a node's input or output.
///
/// This id allow the distinction between :
/// - `in` or "input", where data flowes inward into the node as parameter. An input can only have
///   one edge (connection, source)
/// - `out` or "output", where data flowes outward from the node, as the result of a computation.
///   An output can have multiples edges connected to it, passing data to other node's inputs
///
/// Internally this id is constructed with an [`HashId`], itself constructed as a digest of a
/// `&str` name for an inout
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum InoutId {
    In(HashId),
    Out(HashId),
}

impl InoutId {
    /// Create a new `InoutId::In` based on the given inout name
    pub fn new_in_from(inout_name: &str) -> Self {
        Self::In(HashId::new_from(inout_name))
    }

    /// Create a new `InoutId::Out` based on the given inout name
    pub fn new_out_from(inout_name: &str) -> Self {
        Self::Out(HashId::new_from(inout_name))
    }

    /// Return a [`NodeInoutId`] based on self and the given [`NodeId`]
    pub fn into_node_inout_id(self, node_id: NodeId) -> NodeInoutId {
        NodeInoutId::new(node_id, self)
    }
}

impl Debug for InoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InoutId::In(hash_id) => write!(f, "In({hash_id:?})"),
            InoutId::Out(hash_id) => write!(f, "Out({hash_id:?})"),
        }
    }
}

/// Ties an [`InoutId`] to a [`NodeId`]
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct NodeInoutId {
    node_id: NodeId,
    inout_id: InoutId,
}

impl NodeInoutId {
    pub fn new(node_id: NodeId, inout_id: InoutId) -> Self {
        Self { inout_id, node_id }
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
