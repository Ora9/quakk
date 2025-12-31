use anyhow::{Context, Ok, anyhow};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::{Arc, Mutex},
};

mod meta;
pub use meta::*;

mod id;
pub use id::*;

use crate::{LasyFold, Node};

/// `NodeHandle` is a cheaply cloned reference to a node
///
/// This struct is returned when inserting a [`Node`] into a [`Graph`]
#[derive(Debug, Clone)]
pub struct NodeHandle {
    id: NodeId,
    node: Arc<Box<dyn Node>>,
}

impl NodeHandle {
    /// Given a [`Node`] and its [`NodeId`], return a new `NodeHandle`,
    /// this is not destined to be called by users, but by the [`Graph`] when
    /// inserting a new node
    fn new(node_id: NodeId, node: Box<dyn Node>) -> Self {
        Self {
            id: node_id,
            node: Arc::new(node),
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
pub(crate) struct Vertex {
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

    pub fn inbound_for(&self, in_id: InoutId) -> Option<&NodeInoutId> {
        self.inbound.get(&in_id)
    }

    pub fn outbound_for(&self, out_id: InoutId) -> Option<&HashSet<NodeInoutId>> {
        self.outbound.get(&out_id)
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
}

/// # Node insertion / removal
impl Graph {
    /// Insert a boxed [`Node`] into the graph with the given [`NodeId`], then
    /// return a [`NodeHandle`]
    pub fn insert_with_id(&mut self, node: Box<dyn Node>, node_id: NodeId) -> NodeHandle {
        let node_handle = NodeHandle::new(node_id, node);
        self.vertices
            .insert(node_id, Vertex::new(node_handle.clone()));

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

    /// Does the graph contain a [`Node`] with the given [`NodeId`]
    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertices.contains_key(key)
    }

    pub fn handle_for_id(&self, node_id: NodeId) -> Option<NodeHandle> {
        self.vertices
            .get(&node_id)
            .and_then(|vertex| Some(vertex.node_handle.clone()))
    }

    pub(crate) fn vertex_for_id(&self, node_id: NodeId) -> Option<&Vertex> {
        self.vertices.get(&node_id)
    }

    pub fn graph_in_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphIn)
            .expect("A graph must always have a `GraphIn` node")
    }

    pub fn graph_out_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphOut)
            .expect("A graph must always have a `GraphOut` node")
    }

    pub fn graph_in_id_for(&self, in_name: &str) -> Option<NodeInoutId> {
        self.graph_in_handle().id_for(in_name)
    }

    pub fn graph_out_id_for(&self, out_name: &str) -> Option<NodeInoutId> {
        self.graph_out_handle().id_for(out_name)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn graph_insertion_and_removal() {
//         let mut graph = Graph::new();

//         let number_handle = graph.insert(Box::new(Number::new()));
//         assert!(graph.contains(&number_handle.id));
//         graph.remove(number_handle.id);
//         assert!(!graph.contains(&number_handle.id));
//     }

//     #[test]
//     fn graph_initialized_with_inout() {}
// }

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

    pub fn unpatch(
        &mut self,
        out_id: NodeInoutId,
        in_id: NodeInoutId,
    ) -> Result<(), anyhow::Error> {
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

    pub fn unpatch_inout(&mut self, _inout_id: NodeInoutId) -> Result<(), anyhow::Error> {
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

    fn fold(&self, out_id: InoutId, _lasy_fold: LasyFold, meta: Meta) -> f32 {
        dbg!(self.title());

        dbg!(out_id);
        dbg!(meta);

        Default::default()
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

    fn fold(&self, out_id: InoutId, _lasy_fold: LasyFold, meta: Meta) -> f32 {
        dbg!(self.title());

        dbg!(out_id);
        dbg!(meta);

        Default::default()
    }
}

#[derive(Debug)]
struct Subgraph {
    graph: Arc<Mutex<Graph>>,
}

impl Node for Subgraph {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            graph: Arc::new(Mutex::new(Graph::new())),
        }
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "in" => Some(InoutId::new_in_from(inout_name)),
            "out" => Some(InoutId::new_out_from(inout_name)),
            _ => None,
        }
    }

    fn fold(&self, container_out_id: InoutId, lasy_fold: LasyFold, meta: Meta) -> f32 {
        // if self.id_for("out") == Some(container_out_id) {

        //     let inner_out_id = {
        //         self.graph
        //             .lock()
        //             .expect("the inner graph has been poisoned, who was it ?!")
        //             .graph_out_id_for("out")
        //             .context("out name not found for this graph")
        //             bail
        //     };

        //     let lasy_fold = LasyFold::new(out_id.node_id(), self.graph.clone());

        //     lasy_fold
        //         .get_input(out_id.inout_id(), self.base_meta)
        //         .ok_or(anyhow!("prout"))

        //     lasy_fold.get_input(in_id, meta)

        // } else {
        // }
        Default::default()
    }

    fn title(&self) -> &str {
        "Subgraph"
    }
}

/// # Graph evaluation
impl Graph {
    // pub fn evaluate(&self, out_id: InoutId, lasy_fold: LasyExecutor, meta: Meta) {

    //     let out_handle = self.graph_out_handle();
    //     dbg!(out_id, out_handle);

    //     lasy_fold.get()
    // }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
