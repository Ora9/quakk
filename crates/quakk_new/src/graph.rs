use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{FunctionId, Node, NodeBox, NodeId, NodePortId, PortId, PortLabel, VertexId};

#[derive(Debug)]
pub struct FunctionHandle {
    id: FunctionId,
    name: String,
}

#[derive(Debug)]
pub struct NodeHandle {
    id: NodeId,
    node: Rc<NodeBox>,
}

impl Clone for NodeHandle {
    fn clone(&self) -> Self {
        NodeHandle {
            id: self.id,
            node: self.node.clone(),
        }
    }
}

impl NodeHandle {
    fn new(node_id: NodeId, node: NodeBox) -> Self {
        Self {
            id: node_id,
            node: Rc::new(node),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.id
    }

    pub fn node(&self) -> Rc<NodeBox> {
        self.node.clone()
    }

    pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
        PortId::Node(NodePortId::new(self.id, label.into()))
    }
}

#[derive(Debug)]
enum VertexInner {
    Node(NodeHandle),
    Function(FunctionHandle),
}

#[derive(Debug)]
pub struct Vertex {
    inner: VertexInner,

    inbound: HashMap<PortLabel, PortId>,
    outbound: HashMap<PortLabel, HashSet<PortId>>,
}

impl Vertex {
    pub fn new_node(node: NodeHandle) -> Self {
        Self {
            inner: VertexInner::Node(node),
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn new_function(function: FunctionHandle) -> Self {
        Self {
            inner: VertexInner::Function(function),
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    vertices: HashMap<VertexId, Vertex>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
        }
    }

    pub fn insert_node_with_id(&mut self, node: NodeBox, id: NodeId) -> NodeHandle {
        let node_handle = NodeHandle::new(id, node);
        let vertex = Vertex::new_node(node_handle.clone());
        self.vertices.insert(VertexId::Node(id), vertex);

        node_handle
    }

    pub fn insert_node(&mut self, node: NodeBox) -> NodeHandle {
        let id = NodeId::new_random();
        self.insert_node_with_id(node, id)
    }

    pub fn patch(&mut self, port_out: PortId, port_in: PortId) -> Result<(), anyhow::Error> {
        dbg!(port_out, port_in);

        Ok(())
        // match port_out {
        //     PortId::Node(node_id) =>
        // }
    }
}
