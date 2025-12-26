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

#[derive(Debug, Clone)]
pub struct NodeHandle {
    id: NodeId,
    node: Arc<Box<dyn Node>>,
    // graph: Arc<Mutex<Graph>>,
}

impl NodeHandle {
    pub fn new(
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

    pub fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        self.node.id_for(inout_name, self.id)
    }
}

#[derive(Debug)]
struct Vertex {
    node_handle: NodeHandle,

    inbound: HashMap<InoutId, InoutId>,
    outbound: HashMap<InoutId, HashSet<InoutId>>,
}

impl Vertex {
    fn new(node_handle: NodeHandle) -> Self {
        Self {
            node_handle,

            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }
}

/// A graph contains nodes,
#[derive(Debug)]
pub struct Graph {
    vertices: HashMap<NodeId, Vertex>,
}

/// # Graph creation
impl Graph {
    /// Initilize a new graph, with graph in and out special nodes
    pub fn new() -> Self {
        let mut graph = Self {
            vertices: HashMap::with_capacity(2),
        };

        graph.insert_with_id(Box::new(GraphIn::new()), NodeId::GraphIn);
        graph.insert_with_id(Box::new(GraphOut::new()), NodeId::GraphOut);

        graph
    }

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

    pub fn in_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphIn).unwrap()
    }

    pub fn out_handle(&self) -> NodeHandle {
        self.handle_for_id(NodeId::GraphOut).unwrap()
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
    pub fn patch(&mut self, out_id: InoutId, in_id: InoutId) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(out_id)
            .or_default()
            .insert(in_id);

        self.vertices
            .get_mut(&in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .insert(in_id, out_id);

        Ok(())
    }

    pub fn unpatch(&mut self, out_id: InoutId, in_id: InoutId) -> Result<(), anyhow::Error> {
        self.vertices
            .get_mut(&out_id.node_id())
            .context("The given `out` node does not exists")?
            .outbound
            .entry(out_id)
            .or_default()
            .remove(&in_id);

        self.vertices
            .get_mut(&in_id.node_id())
            .context("The given `in` node does not exists")?
            .inbound
            .remove(&in_id);

        Ok(())
    }

    pub fn unpatch_inout(&mut self, inout_id: InoutId) -> Result<(), anyhow::Error> {
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

    fn id_for(&self, inout_name: &str, node_id: NodeId) -> Option<InoutId> {
        match inout_name {
            "number_in" => Some(InoutId::Out(node_id, HashId::new_with("number_in"))),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "GraphIn"
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);
    }
}

#[derive(Debug)]
pub struct GraphOut;

impl Node for GraphOut {
    fn new() -> Self {
        Self
    }

    fn id_for(&self, inout_name: &str, node_id: NodeId) -> Option<InoutId> {
        match inout_name {
            "number_out" => Some(InoutId::Out(node_id, HashId::new_with("number_out"))),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "GraphOut"
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);
    }
}

// /// # Graph ins and outs
// impl Graph {
//     pub fn graph_in_handle(&self) -> NodeHandle {

//     }


// }

/// # Graph evaluation
impl Graph {
    pub fn evaluate(&self) {
        // for (id, node) in &self.nodes {
        //         node.evaluate(None, Box::new("oui!".to_string()), Meta {
        //             quality: Quality::Balanced,
        //             tick: 5
        //         });
        //     }
        // }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

struct LasyInputs {
    node_id: NodeId,
    graph: Arc<Mutex<Graph>>,
}

impl LasyInputs {
    fn get() {}
}
