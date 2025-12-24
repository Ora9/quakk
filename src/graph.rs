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
    node_handle: Option<NodeHandle>,

    inbound: HashMap<InoutId, InoutId>,
    outbount: HashMap<InoutId, HashSet<InoutId>>,
}

impl Vertex {
    fn new_node(node_handle: NodeHandle) -> Self {
        Self::new(Some(node_handle))
    }

    fn new(node_handle: Option<NodeHandle>) -> Self {
        Self {
            node_handle: node_handle,

            inbound: HashMap::new(),
            outbount: HashMap::new(),
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
    pub fn new() -> Self {
        let mut graph = Self {
            vertices: HashMap::with_capacity(2),
        };

        graph.insert_maybe_node_with_id(None, NodeId::GraphIn);
        graph.insert_maybe_node_with_id(None, NodeId::GraphOut);

        graph
    }

    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertices.contains_key(key)
    }
}

/// # Node insertion / removal
impl Graph {
    fn insert_maybe_node_with_id(
        &mut self,
        maybe_node: Option<Box<dyn Node>>,
        node_id: NodeId,
    ) -> Option<NodeHandle> {
        let node_handle_opt = if let Some(node) = maybe_node {
            let node_handle = NodeHandle::new(node_id, node);
            Some(node_handle)
        } else {
            None
        };

        self.vertices
            .insert(node_id, Vertex::new(node_handle_opt.clone()));
        node_handle_opt
    }

    /// Insert a boxed [`Node`] into the graph with the given [`NodeId`], then
    /// return a [`NodeHandle`]
    pub fn insert_with_id(&mut self, node: Box<dyn Node>, node_id: NodeId) -> NodeHandle {
        Self::insert_maybe_node_with_id(self, Some(node), node_id)
            .expect("When passing a `Node`, we should always get a `NodeHandle`")
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
    fn graph_initialized_with_inout() {

    }
}

/// # Graph patching
impl Graph {
    pub fn patch_force(&mut self, out_id: InoutId, in_id: InoutId) -> Result<(), anyhow::Error> {
        Ok(())
    }

    pub fn patch(&mut self, out_id: InoutId, in_id: InoutId) -> Result<(), anyhow::Error> {
        dbg!(out_id, in_id);

        // get two vertices
        // make all check before changing anything
        //  - is "in" empty?
        // add in_id to out_id outbound set
        // add out_id to in_id
        //

        let out_vertex = self
            .vertices
            .get(&out_id.node_id())
            .context("The given `out` node does not exists")?;

        let in_vertex = self
            .vertices
            .get(&in_id.node_id())
            .context("The given `in` node does not exists")?;

        // let mut in_vertex = self.vertices.get_mut(&in_id.node_id());

        // dbg!(in_vertex, out_vertex);

        Ok(())
    }

    pub fn unpatch(&mut self, out_id: InoutId, in_id: InoutId) -> Result<(), anyhow::Error> {
        dbg!(out_id, in_id);
        Ok(())
    }

    pub fn unpatch_inout(&mut self, inout_id: InoutId) -> Result<(), anyhow::Error> {
        Ok(())
    }

    // unpatch node
    // unpatch nodes
}

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
