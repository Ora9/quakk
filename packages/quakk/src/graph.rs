use anyhow::{Context, Ok, anyhow};
use std::{
    any::{self, Any},
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::Sub,
    sync::{Arc, Mutex},
};

use crate::{
    Data, LasyFold, Meta, Node,
    id::{InId, InoutId, NodeId, NodeInId, NodeInoutId, NodeOutId, OutId},
    numeric::{ArithmeticsInId, NumericConstantOutId},
};

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

    // /// Given a string identifier, will return an [`InoutId`] if the node recognise
    // /// it as a valid in/out name
    // ///
    // /// The format and convention around said identifier is not formalised yet,
    // /// but will be eventually
    // pub fn id_for(&self, inout_name: &str) -> Option<NodeInoutId> {
    //     self.node.node_inout_id_for(inout_name, self.id)
    // }
    pub fn node_in_id(&self, in_id: &dyn InId) -> Option<NodeInId> {
        self.node().node_in_id(in_id, self.node_id())
    }

    pub fn node_out_id(&self, out_id: &dyn OutId) -> Option<NodeOutId> {
        self.node().node_out_id(out_id, self.node_id())
    }

    // pub fn in_id_for(&self, in_name: &str) -> Option<NodeInId> {
    //     self.node
    //         .in_id_for(in_name)
    //         .map(|in_id| NodeInId::new(self.node_id(), in_id))
    // }

    // pub fn out_id_for(&self, out_name: &str) -> Option<NodeOutId> {
    //     self.node
    //         .out_id_for(out_name)
    //         .map(|out_id| NodeOutId::new(self.node_id(), out_id))
    // }
}

/// `Vertex` is an item in the graph, it holds a [`NodeHandle`], but also all
/// inbound and outbound connection of the node
#[derive(Debug)]
pub(crate) struct Vertex {
    node_handle: NodeHandle,

    inbound: HashMap<Box<dyn InId>, NodeOutId>,
    outbound: HashMap<Box<dyn OutId>, HashSet<NodeInId>>,
}

impl Vertex {
    fn new(node_handle: NodeHandle) -> Self {
        Self {
            node_handle,

            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn inbound_for(&self, in_id: &dyn InId) -> Option<&NodeOutId> {
        self.inbound.get(&dyn_clone::clone_box(in_id))
    }

    pub fn outbound_for(&self, out_id: &dyn OutId) -> Option<&HashSet<NodeInId>> {
        self.outbound.get(&dyn_clone::clone_box(out_id))
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

    pub fn graph_out_in_id(&self, in_id: &dyn InId) -> Option<NodeInId> {
        self.graph_out_handle().node_in_id(in_id)
    }

    pub fn graph_in_out_id(&self, out_id: &dyn OutId) -> Option<NodeOutId> {
        self.graph_in_handle().node_out_id(out_id)
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
    pub fn patch(
        &mut self,
        node_out_id: NodeOutId,
        node_in_id: NodeInId,
    ) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&node_out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(node_out_id.clone().out_id())
            .or_default()
            .insert(node_in_id.clone());

        self.vertices
            .get_mut(&node_in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .insert(node_in_id.in_id(), node_out_id);

        Ok(())
    }

    pub fn unpatch(
        &mut self,
        node_out_id: NodeOutId,
        node_in_id: NodeInId,
    ) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&node_out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(node_out_id.out_id())
            .or_default()
            .remove(&node_in_id);

        self.vertices
            .get_mut(&node_in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .remove(&node_in_id.in_id());

        Ok(())
    }

    pub fn unpatch_inout(&mut self, _inout_id: NodeInoutId) -> Result<(), anyhow::Error> {
        Ok(())
    }

    // unpatch node
    // unpatch nodes
}

#[derive(Debug, Default)]
pub struct GraphIn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphInOutId {
    Numeric,
}

impl OutId for GraphInOutId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphInInId {
    Numeric,
}

impl InId for GraphInOutId {}

impl GraphIn {
    pub fn new() -> Self {
        Self
    }
}

impl Node for GraphIn {
    fn initialize() -> Self {
        Self::default()
    }

    // fn id_for(&self, inout_name: &str) -> Option<InoutId> {
    //     match inout_name {
    //         "numeric" => Some(InoutId::new_out_from("number_in")),
    //         _ => None,
    //     }
    // }

    fn title(&self) -> &str {
        "GraphIn"
    }

    fn fold(&self, out_id: &dyn OutId, _lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<Data> {
        dbg!(self.title());

        dbg!(out_id);
        dbg!(meta);

        Ok(Data::new(f32::default()))
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        if let Some(out_id) = out_id.as_any().downcast_ref::<GraphInOutId>() {
            Some(NodeOutId::new(node_id, out_id))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct GraphOut;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphOutInId {
    Numeric,
}

impl InId for GraphOutInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphOutOutId {
    Numeric,
}

impl OutId for GraphOutOutId {}

impl GraphOut {
    pub fn new() -> Self {
        Self
    }
}

impl Node for GraphOut {
    fn initialize() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "GraphOut"
    }

    fn fold(&self, out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<Data> {
        dbg!(out_id);

        if let Some(out_id) = out_id.as_any().downcast_ref::<GraphOutOutId>() {
            match out_id {
                GraphOutOutId::Numeric => lasy_fold.get_in(&GraphOutInId::Numeric, meta),
            }
        } else {
            Err(anyhow!("not a valid out_id"))
        }
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        if let Some(in_id) = in_id.as_any().downcast_ref::<GraphOutInId>() {
            Some(NodeInId::new(node_id, in_id))
        } else {
            None
        }
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        None
    }
}

#[derive(Debug)]
struct Subgraph {
    graph: Arc<Mutex<Graph>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubgraphInId {
    name: String,
}

impl InId for SubgraphInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubgraphOutId {
    name: String,
}

impl OutId for SubgraphOutId {}

impl Subgraph {
    pub fn new() -> Self {
        Self {
            graph: Arc::new(Mutex::new(Graph::new())),
        }
    }
}

impl Node for Subgraph {
    fn initialize() -> Self {
        Self::default()
    }

    fn fold(
        &self,
        container_out_id: &dyn OutId,
        lasy_fold: LasyFold,
        meta: Meta,
    ) -> anyhow::Result<Data> {
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
        Ok(Data::new(f32::default()))
    }

    fn title(&self) -> &str {
        "Subgraph"
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        if let Some(in_id) = in_id.as_any().downcast_ref::<SubgraphInId>() {
            Some(NodeInId::new(node_id, in_id))
        } else {
            None
        }
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        if let Some(out_id) = out_id.as_any().downcast_ref::<SubgraphOutId>() {
            Some(NodeOutId::new(node_id, out_id))
        } else {
            None
        }
    }
}

impl Default for Subgraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
