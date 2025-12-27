use anyhow::{Context, Ok, anyhow};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
};

mod meta;
pub use meta::*;

pub mod node;
pub use node::Node;

mod id;
pub use id::*;

use crate::{LasyExecutor};

/// `NodeHandle` is a cheaply cloned reference to a node
///
/// This struct is returned when inserting a [`Node`] into a [`Graph`]
#[derive(Debug, Clone)]
pub struct NodeHandle {
    id: NodeId,
    node: Arc<Box<dyn Node>>,
    // graph: Arc<Mutex<Graph>>,
}

impl NodeHandle {
    /// Given a [`Node`] and its [`NodeId`], return a new `NodeHandle`,
    /// this is not destined to be called by users, but by the [`Graph`] when
    /// inserting a new node
    fn new(
        node_id: NodeId,
        node: Box<dyn Node>,
        // graph: Arc<Mutex<Graph>>
    ) -> Self {
        Self {
            id: node_id,
            node: Arc::new(node),
            // graph,
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.id
    }

    pub fn node(&self) -> Arc<Box<dyn Node>> {
        self.node.clone()
    }

    /// Given a string identifier, will return an [`InoutId`] if the node recognise
    /// it as a valid in/out name
    ///
    /// The format and convention around said identifier is not formalised yet,
    /// but will be eventually
    pub fn id_for(&self, inout_name: &str) -> Option<NodeInoutId> {
        self.node.node_inout_id_for(inout_name, self.id)
    }
}

/// `Vertex` is an item in the graph, it holds a [`NodeHandle`], but also all
/// inbound and outbound connection of the node
#[derive(Debug)]
struct Vertex {
    node_handle: NodeHandle,

    inbound: HashMap<InoutId, NodeInoutId>,
    outbound: HashMap<InoutId, HashSet<NodeInoutId>>,
}

impl Vertex {
    fn new(node_handle: NodeHandle) -> Self {
        Self {
            node_handle,

            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    fn inbound(self) -> HashMap<InoutId, NodeInoutId> {
        self.inbound
    }

    fn outbound(self) -> HashMap<InoutId, HashSet<NodeInoutId>> {
        self.outbound
    }

    fn inbound_for(&self, inout_id: InoutId) -> Option<&NodeInoutId> {
        self.inbound.get(&inout_id)
    }

    fn outbound_for(&self, inout_id: InoutId) -> Option<&HashSet<NodeInoutId>> {
        self.outbound.get(&inout_id)
    }
}

/// A `Graph` hold nodes and handle all connections (patches)
#[derive(Debug)]
pub struct Graph {
    vertices: HashMap<NodeId, Vertex>,
}

/// # Graph creation
impl Graph {
    /// Return a new and initialized graph, holding two specials [`Node`]s :
    /// `GraphIn` and `GraphOut`
    pub fn new() -> Self {
        let mut graph = Self {
            vertices: HashMap::with_capacity(2),
        };

        graph.insert_with_id(Box::new(GraphIn::new()), NodeId::GraphIn);
        graph.insert_with_id(Box::new(GraphOut::new()), NodeId::GraphOut);

        graph
    }

    /// Does the graph contain a [`Node`] with the given [`NodeId`]
    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertices.contains_key(key)
    }
}

/// # Node insertion / removal
impl Graph {
    /// Insert a boxed [`Node`] into the graph with the given [`NodeId`], then
    /// return a [`NodeHandle`]
    pub fn insert_with_id(&mut self, node: Box<dyn Node>, node_id: NodeId) -> NodeHandle {
        let node_handle = NodeHandle::new(node_id, node);
        self.vertices.insert(node_id, Vertex::new(node_handle.clone()));

        node_handle
    }

    /// Insert a boxed [`Node`] into the graph, giving it a new random id, then
    /// return a [`NodeHandle`]
    pub fn insert(&mut self, node: Box<dyn Node>) -> NodeHandle {
        let node_id = NodeId::new_node();
        self.insert_with_id(node, node_id)
    }

    pub fn remove(&mut self, node_id: NodeId) -> Result<(), anyhow::Error> {
        match node_id {
            NodeId::GraphIn | NodeId::GraphOut => Err(anyhow!("Cannot remove the graph in or out")),
            _ => {
                self.vertices.remove(&node_id);
                Ok(())
            }
        }
    }

    pub fn handle_for_id(&self, node_id: NodeId) -> Option<NodeHandle> {
        self.vertices.get(&node_id).and_then(|vertex| {
            Some(vertex.node_handle.clone())
        })
    }

    pub fn graph_in_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphIn).expect("A graph must always have a `GraphIn` node")
    }

    pub fn graph_out_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphOut).expect("A graph must always have a `GraphOut` node")

    }

    pub fn graph_in_id_for(&self, in_name: &str) -> Option<NodeInoutId> {
        self.graph_in_handle().id_for(in_name)
    }

    pub fn graph_out_id_for(&self, out_name: &str) -> Option<NodeInoutId> {
        self.graph_out_handle().id_for(out_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Number;

    use super::*;

    #[test]
    fn graph_insertion_and_removal() {
        let mut graph = Graph::new();

        let number_handle = graph.insert(Box::new(Number::new()));
        assert!(graph.contains(&number_handle.id));
        graph.remove(number_handle.id);
        assert!(!graph.contains(&number_handle.id));
    }

    #[test]
    fn graph_initialized_with_inout() {}
}

/// # Graph patching
impl Graph {
    pub fn patch(&mut self, out_id: NodeInoutId, in_id: NodeInoutId) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(out_id.inout_id())
            .or_default()
            .insert(in_id);

        self.vertices
            .get_mut(&in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .insert(in_id.inout_id(), out_id);

        Ok(())
    }

    pub fn unpatch(&mut self, out_id: NodeInoutId, in_id: NodeInoutId) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(out_id.inout_id())
            .or_default()
            .remove(&in_id);

        self.vertices
            .get_mut(&in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .remove(&in_id.inout_id());

        Ok(())
    }

    pub fn unpatch_inout(&mut self, inout_id: NodeInoutId) -> Result<(), anyhow::Error> {
        Ok(())
    }

    // unpatch node
    // unpatch nodes
}

#[derive(Debug)]
pub struct GraphIn;

impl Node for GraphIn {
    fn new() -> Self {
        Self
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "number_in" => Some(InoutId::new_in_from("number_in")),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "GraphIn"
    }

    fn evaluate(&self, out_id: Option<InoutId>, lasy_executor: LasyExecutor, meta: Meta) {
        dbg!(self.title());

        dbg!(out_id);
        dbg!(meta);
    }
}

#[derive(Debug)]
pub struct GraphOut;

impl Node for GraphOut {
    fn new() -> Self {
        Self
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "number_out" => Some(InoutId::new_in_from("number_out")),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "GraphOut"
    }

    fn evaluate(&self, out_id: Option<InoutId>, lasy_executor: LasyExecutor, meta: Meta) {
        dbg!(self.title());

        dbg!(out_id);
        dbg!(meta);
    }
}

/// # Graph evaluation
impl Graph {
    pub fn evaluate(&self, out_id: Option<InoutId>, lasy_executor: LasyExecutor, meta: Meta) {

        let out_handle = self.graph_out_handle();
        dbg!(out_id, out_handle);

        // self.graph_out_handle().node().evaluate(output_id, input, meta);
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
