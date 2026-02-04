//! Quakk uses a few differents types of identifiers for nodes and their ins and outs :
//! - [`NodeId`] : Identifies a unique node in a graph
//! - [`InoutId`] : Identifies either an input or output of an anonymous node. The specific node it
//!   is tied to is not specified and should be unambiguously determined from context if needed
//!     - [`InId`] : An input of an anonymous node
//!     - [`OutId`] : An output of an anonymous node
//! - [`NodeInoutId`] : Identifies either an input or output of a specific node
//!     - [`NodeInId`] : An input of a node
//!     - [`NodeOutId`] : An input of a node
//!
//! All of these eventually rely on [`HashId`] which is a simple hash, either randomly determined,
//! or based on a string
//!
//! ```text
//!   ┌────────┐ ┌─────────┐ ┌───────────────┐
//!   │ NodeId │ │ InoutId │ │ NodeInoutId   │
//!   └─┬──────┘ └─┬─────┬─┘ └─┬───────────┬─┘
//!     │          │     │   ┌─▼────────┐┌─▼─────────┐
//!     │          │     └─┐ │ NodeInId ││ NodeOutId │
//!     │          │       │ └─┬────────┘└─┬─────────┘
//!     │          ├───────┼───┘           │
//!     │          │       ├───────────────┘
//!     │        ┌─▼────┐┌─▼─────┐
//!     │        │ InId ││ OutId │
//!     │        └─┬────┘└┬──────┘
//!     ├──────────┴──────┘
//!   ┌─▼──────┐
//!   │ HashId │
//!   └────────┘
//! ```
use std::{
    any::Any,
    fmt::Debug,
    hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState},
};

use anyhow::anyhow;
use dyn_clone::DynClone;
use dyn_eq::DynEq;
use dyn_hash::DynHash;

/// A simple hash, used by [`NodeId`], [`InId`] and [`OutId`]
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

/// A id used to identify a [`Node`](quakk::Node)
///
/// It allows representing `GraphIn` and `GraphOut`. Thoses are specials types of nodes
/// that handles ins and outs for the [`Graph`](quakk::Graph) itself. Theses specials nodes always
/// exist once each in a graph.
///
/// Other nodes are identified with an [`HashId`], usually random
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

pub trait InId: Any + Debug + DynClone + DynEq + DynHash {}
dyn_clone::clone_trait_object!(InId);
dyn_eq::eq_trait_object!(InId);
dyn_hash::hash_trait_object!(InId);

pub trait OutId: Any + Debug + DynClone + DynEq + DynHash {}
dyn_clone::clone_trait_object!(OutId);
dyn_eq::eq_trait_object!(OutId);
dyn_hash::hash_trait_object!(OutId);

/// In the [`Graph`](quakk::Graph), each [`Nodes`](quakk::Node) ins and outs have an id.
///
/// The term `inout` is widely used in the code and documentation to refer to a node's input or output.
///
/// `InoutId` is designed to uniquely indentify an inout in an unspecified node, it is *not* tied to
/// specific node and as a result cannot uniquely identify an inout in the graph. That would be the
/// purpose of [`NodeInoutId`]
///
/// `InoutId` should be used only where the [`NodeId`] is unimportant, or can be unambiguously
/// determined by context.
///
/// This id allow the distinction between :
/// - `in` or "input", where data flowes inward into the node as parameter. An input can only have
///   one edge (connection between a node's out and another node's in)
/// - `out` or "output", where data flowes outward from the node, as the result of a computation.
///   An output can have multiples edges connected to it, passing data to other node's inputs
///
/// Internally this id is constructed as an enum of either [`InId`] or [`OutId`]
#[derive(PartialEq, Eq, Clone, Hash)]
pub enum InoutId {
    In(Box<dyn InId>),
    Out(Box<dyn OutId>),
}

impl Debug for InoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InoutId::In(in_id) => write!(f, "{:?}", in_id),
            InoutId::Out(out_id) => write!(f, "{:?}", out_id),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct NodeInId {
    node_id: NodeId,
    in_id: Box<dyn InId>,
}

impl NodeInId {
    pub fn new(node_id: NodeId, in_id: &dyn InId) -> Self {
        Self {
            node_id,
            in_id: dyn_clone::clone_box(in_id),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn in_id(self) -> Box<dyn InId> {
        self.in_id
    }
}

impl Debug for NodeInId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}>{:?}", self.node_id, self.in_id)
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct NodeOutId {
    node_id: NodeId,
    out_id: Box<dyn OutId>,
}

impl NodeOutId {
    pub fn new(node_id: NodeId, out_id: &dyn OutId) -> Self {
        Self {
            node_id,
            out_id: dyn_clone::clone_box(out_id),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn out_id(self) -> Box<dyn OutId> {
        self.out_id
    }
}

impl Debug for NodeOutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}>{:?}", self.node_id, self.out_id)
    }
}

/// In the [`Graph`](quakk::Graph), each [`Nodes`](quakk::Node) ins and outs have an id.
///
/// This id is designed to identify an inout ("in" or "out") in the graph
/// It ties
#[derive(PartialEq, Eq, Clone)]
pub enum NodeInoutId {
    In(NodeInId),
    Out(NodeOutId),
}

impl NodeInoutId {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::In(node_in_id) => node_in_id.node_id,
            Self::Out(node_out_id) => node_out_id.node_id,
        }
    }
}

impl Debug for NodeInoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In(node_in_id) => write!(f, "{:?}", self),
            Self::Out(node_out_id) => write!(f, "{:?}", self),
        }
    }
}

impl From<NodeInId> for NodeInoutId {
    fn from(value: NodeInId) -> Self {
        Self::In(value)
    }
}

impl From<NodeOutId> for NodeInoutId {
    fn from(value: NodeOutId) -> Self {
        Self::Out(value)
    }
}
