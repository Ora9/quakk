use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{FunctionId, Node, NodeBox, NodeId, NodePortId, PortId, PortLabel, VertexId};

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    color: u32,
}

#[derive(Debug, Clone)]
pub struct FunctionHandle {
    id: FunctionId,
    function: Function,
}

impl FunctionHandle {
    fn new(function_id: FunctionId, function: Function) -> FunctionHandle {
        FunctionHandle {
            id: function_id,
            function,
        }
    }
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
    pub fn new_node(node_handle: NodeHandle) -> Self {
        Self {
            inner: VertexInner::Node(node_handle),
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    pub fn new_function(function_handle: FunctionHandle) -> Self {
        Self {
            inner: VertexInner::Function(function_handle),
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    vertices: HashMap<VertexId, Vertex>,
    main_function_id: Option<FunctionId>,
}

impl Graph {
    pub fn new() -> Self {
        let mut graph = Graph {
            vertices: HashMap::new(),
            main_function_id: None,
        };

        let main = Function {
            name: "Main".to_string(),
            color: 0,
        };

        let main_handle = graph.insert_function(main);
        graph.main_function_id = Some(main_handle.id);

        graph
    }

    pub fn insert_node(&mut self, node: NodeBox) -> NodeHandle {
        let id = NodeId::new_random();
        let node_handle = NodeHandle::new(id, node);
        let vertex = Vertex::new_node(node_handle.clone());
        self.vertices.insert(VertexId::Node(id), vertex);

        node_handle
    }

    pub fn insert_function(&mut self, function: Function) -> FunctionHandle {
        let id = FunctionId::new();
        let function_handle = FunctionHandle::new(id, function);

        let vertex = Vertex::new_function(function_handle.clone());
        self.vertices.insert(VertexId::Function(id), vertex);

        function_handle
    }

    pub fn patch(&mut self, port_out: PortId, port_in: PortId) -> Result<(), anyhow::Error> {
        let vertex_out = VertexId::from_port_id(&port_out);
        let vertex_in = VertexId::from_port_id(&port_in);

        dbg!(port_out, vertex_out);
        dbg!(port_in, vertex_in);

        dbg!(self.vertices.get_mut(&vertex_in));

        // self.vertices.get_mut(k)

        Ok(())
        // match port_out {
        //     PortId::Node(node_id) =>
        // }
    }
}
