use std::{
    any::Any,
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

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
        id: NodeId,
        node: Box<dyn Node>,
        // graph: Arc<Mutex<Graph>>
    ) -> Self {
        Self {
            id,
            node: Arc::new(node),
            // graph,
        }
    }

    pub fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        self.node.id_for(inout_name, self.id)
    }
}

// #[derive(Debug)]
// struct Edge {
//     source: InoutId,
//     target: Vec<InoutId>,
// }

#[derive(Debug)]
struct Vertex {
    node: NodeHandle,

    inbound: HashMap<InoutId, InoutId>,
    outbount: HashMap<InoutId, HashSet<InoutId>>,
}

impl Vertex {
    pub fn new(node: NodeHandle) -> Self {
        Self {
            node,

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
        Self {
            vertices: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: HashMap::with_capacity(capacity),
        }
    }

    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertices.contains_key(key)
    }
}

/// # Node management
impl Graph {
    pub fn insert(&mut self, node: Box<dyn Node>) -> NodeHandle {
        let id = NodeId::new_node();
        let node_handle = NodeHandle::new(id, node);

        self.vertices.insert(id, Vertex::new(node_handle.clone()));

        node_handle
    }

    pub fn patch(&mut self, output_edgepoint: InoutId, input_edgepoint: InoutId) {
        // self.edges.insert()

        dbg!(output_edgepoint, input_edgepoint);
    }
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
