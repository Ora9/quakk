use std::{any::Any, collections::{HashMap, HashSet}, fmt::Debug, hash::Hash, sync::{Arc, Mutex}};
use uuid::Uuid;

mod meta;
pub use meta::*;

pub mod node;
pub use node::Node;

mod id;
pub use id::*;

#[derive(Debug)]
struct NodeHandle {
    node: Arc<Box<dyn Node>>,
}

impl NodeHandle {
    pub fn new(node: Box<dyn Node>) -> Self {
        Self {
            node: Arc::new(node),
        }
    }
}

// #[derive(Debug)]
// struct Edge {
//     source: InoutId,
//     target: Vec<InoutId>,
// }

/// A vertex is a conceptual representation of either :
/// - A node
/// - The graph inputs or outputs
///
/// This is well represented by [`VertexId`]
#[derive(Debug)]
struct Vertex {
    node: NodeHandle,

    inbound: HashMap<InoutId, InoutId>,
    outbount: HashMap<InoutId, Vec<InoutId>>,
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

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vertices: HashMap::with_capacity(capacity)
        }
    }

    pub fn contains(&self, key: &NodeId) -> bool {
        self.vertices.contains_key(key)
    }
}

impl Graph {
    pub fn insert(&mut self, node: Box<dyn Node>) -> NodeId {
        let id = NodeId::new();
        self.vertices.insert(
            id,
            Vertex::new(NodeHandle::new(node))
        );

        id
    }

    pub fn patch(&mut self, output_edgepoint: InoutId, input_edgepoint: InoutId) {

        // self.edges.insert()

        dbg!(output_edgepoint, input_edgepoint);
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
    node_id: VertexId,
    graph: Arc<Mutex<Graph>>,
}

impl LasyInputs {
    fn get() {

    }
}
