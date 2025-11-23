use std::{any::Any, collections::{HashMap, HashSet}, fmt::Debug, hash::Hash, sync::{Arc, Mutex}};
use uuid::Uuid;

mod meta;
pub use meta::*;

pub mod node;
pub use node::Node;

mod id;
pub use id::*;

#[derive(Debug)]
struct GraphEdge {
    source: Vertex,
    target: Vertex,
}

#[derive(Debug)]
struct EdgepointRef {
    id: EdgepointId,
    vertex: Arc<Vertex>,
}

impl EdgepointRef {
    pub fn clone(&self) -> Self {
        Self {
            id: self.id,
            vertex: self.vertex.clone()
        }
    }
}

#[derive(Debug)]
struct Vertex {
    node: Box<dyn Node>,

    inbound: HashMap<EdgepointRef, EdgepointRef>,
    outbount: HashMap<EdgepointRef, Vec<EdgepointRef>>,
}

impl Vertex {
    pub fn new(node: Box<dyn Node>) -> Self {
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
    /// Nodes
    vertex: HashMap<NodeId, Arc<Vertex>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertex: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertex: HashMap::with_capacity(capacity)
        }
    }

    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertex.contains_key(key)
    }
}

impl Graph {
    pub fn insert(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = NodeId::new_node();
        let vertex = Vertex::new(node);

        self.vertex.insert(id, Arc::new(vertex));
        id
    }

    pub fn patch(&mut self, output_edgepoint: EdgepointId, input_edgepoint: EdgepointId) {

        // self.edges.insert()

        dbg!(matches!(output_edgepoint, EdgepointId::GraphInput(_)));

    }

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

struct LasyInputs {
    node_id: NodeId,
    graph: Arc<Mutex<Graph>>,
}

impl LasyInputs {
    fn get() {

    }
}
