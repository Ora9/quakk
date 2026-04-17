use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use anyhow::{Context, anyhow};

use crate::{FunctionId, Node, NodeId, PortLabel, VertexId, VertexPortId};

// #[derive(Debug)]
// pub struct NodeHandle {
//     id: NodeId,
//     // node: Rc<NodeBox>,
// }

// impl Clone for NodeHandle {
//     fn clone(&self) -> Self {
//         NodeHandle {
//             id: self.id,
//             // node: self.node.clone(),
//         }
//     }
// }

// impl NodeHandle {
//     fn new(node_id: NodeId) -> Self {
//         Self {
//             id: node_id,
//             // node: Rc::new(node),
//         }
//     }
//     pub fn node_id(&self) -> NodeId {
//         self.id
//     }

//     // pub fn node(&self) -> Rc<NodeBox> {
//     //     self.node.clone()
//     // }

//     pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
//         PortId::Node(NodePortId::new(self.id, label.into()))
//     }

//     pub fn out(&self) -> PortId {
//         self.port_id("out")
//     }

//     pub fn r#in(&self) -> PortId {
//         self.port_id("in")
//     }
// }

// #[derive(Debug, Clone)]
// pub struct FunctionHandle {
//     id: FunctionId,
//     // function: Function,
// }

// impl FunctionHandle {
//     pub fn new(function_id: FunctionId) -> FunctionHandle {
//         FunctionHandle {
//             id: function_id,
//             // function,
//         }
//     }

//     pub fn id(&self) -> FunctionId {
//         self.id
//     }

//     // pub fn function(&self) -> Function {
//     //     self.function.clone()
//     // }

//     pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
//         PortId::Function(FunctionPortId::new(self.id, label.into()))
//     }

//     // fn fold_for(&self, graph: Graph, label: impl Into<PortLabel>) -> Result<DataBox, anyhow::Error> {

//     //     self.function.
//     // }
// }

// #[derive(Debug, Clone)]
// enum VertexInner {
//     Node(Rc<Node>),
//     Function(Function),
// }

#[derive(Debug)]
pub struct Vertex {
    node: Node,

    inbound: HashMap<PortLabel, VertexPortId>,
    outbound: HashMap<PortLabel, HashSet<VertexPortId>>,
}

impl Vertex {
    fn new(node: Node) -> Self {
        Self {
            node,
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }
    // pub fn new_function(function: Function) -> Self {
    //     Self {
    //         inner: VertexInner::Function(function),
    //         inbound: HashMap::new(),
    //         outbound: HashMap::new(),
    //     }
    // }

    // pub fn node(&self) -> Result<NodeBox, anyhow::Error> {
    //     match &self.inner {
    //         VertexInner::Node(node) => Ok(node.clone()),
    //     }
    // }

    // pub fn node_handle(&self) -> Result<NodeHandle, anyhow::Error> {
    //     match &self.inner {
    //         VertexInner::Node(node_handle) => Ok(node_handle.clone()),
    //         _ => Err(anyhow!("this vertex is not a node")),
    //     }
    // }

    // pub fn function_handle(&self) -> Result<FunctionHandle, anyhow::Error> {
    //     match &self.inner {
    //         VertexInner::Function(function_handle) => Ok(function_handle.clone()),
    //         _ => Err(anyhow!("this vertex is not a function")),
    //     }
    // }

    pub fn outbound_for(&self, port_label: impl Into<PortLabel>) -> Option<HashSet<VertexPortId>> {
        self.outbound.get(&port_label.into()).cloned()
    }

    pub fn inbound_for(&self, port_label: impl Into<PortLabel>) -> Option<VertexPortId> {
        self.inbound.get(&port_label.into()).cloned()
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub color: u32,

    nodes: HashMap<NodeId, Vertex>,
    last_node_id: Option<NodeId>,
}

impl Function {
    pub fn new(name: &str, color: u32) -> Self {
        Self {
            name: name.to_string(),
            color,

            nodes: HashMap::new(),
            last_node_id: None,
        }
    }

    fn next_node_id(&mut self, function_id: FunctionId) -> NodeId {
        let next_node_id = self
            .last_node_id
            .unwrap_or(NodeId::zero(function_id))
            .checked_increment()
            .expect("node_id has overflown: too much nodes");

        self.last_node_id = Some(next_node_id);

        if !self.nodes.contains_key(&next_node_id) {
            next_node_id
        } else {
            eprintln!(
                "failed attempt to create a new id in function #{:?} (id already exists), retrying",
                function_id
            );
            self.next_node_id(function_id)
        }
    }

    fn insert(&mut self, node_id: NodeId, node: Node) {
        self.nodes.insert(node_id, Vertex::new(node));
    }
}

/// A `Graph` hold nodes and handle patches (connection between nodes)
#[derive(Debug)]
pub struct Graph {
    functions: HashMap<FunctionId, Function>,
    // vertices: HashMap<VertexId, Vertex>,
    main_function_id: Option<FunctionId>,
    last_function_id: Option<FunctionId>,
}

impl Graph {
    /// Create a new and initialized graph, only holding a main function
    pub fn new() -> Self {
        let mut graph = Graph {
            functions: HashMap::new(),
            main_function_id: None,
            last_function_id: None,
        };

        let main = Function::new("Main", 55);
        graph.main_function_id = Some(graph.insert_function(main));

        graph
    }

    fn next_function_id(&mut self) -> FunctionId {
        let next_function_id = self
            .last_function_id
            .unwrap_or(FunctionId::ZERO)
            .checked_increment()
            .expect("function_id has overflown: too much functions");

        self.last_function_id = Some(next_function_id);

        if !self.functions.contains_key(&next_function_id) {
            next_function_id
        } else {
            eprintln!("failed attempt to create a new id the graph (id already exists), retrying");
            self.next_function_id()
        }
    }

    /// Insert the given node in the main function
    pub fn insert(&mut self, node: Node) -> NodeId {
        // let node_id = NodeId::new_random(self.main_function_id());
        let main_function_id = self.main_function_id();
        // let main_function = self.main_function_mut();

        self.insert_into(main_function_id, node)
            .expect("should always be able to insert into main_function")

        // let node_id = main_function.next_node_id(main_function_id);
        // main_function.insert(node_id, node);

        // node_id
    }

    /// Insert the given node into the specified function
    pub fn insert_into(
        &mut self,
        function_id: FunctionId,
        node: Node,
    ) -> Result<NodeId, anyhow::Error> {
        let function = self
            .functions
            .get_mut(&function_id)
            .context("this function could not be found")?;

        let node_id = function.next_node_id(function_id);

        function.insert(node_id, node);

        Ok(node_id)
    }

    pub fn insert_function(&mut self, function: Function) -> FunctionId {
        let function_id = self.next_function_id();

        self.functions.insert(function_id, function);

        function_id
    }

    pub fn patch(
        &mut self,
        port_out: VertexPortId,
        port_in: VertexPortId,
    ) -> Result<(), anyhow::Error> {
        let vertex_out = VertexId::from_port_id(&port_out);
        let vertex_in = VertexId::from_port_id(&port_in);

        // self.vertices
        //     .get_mut(&vertex_out)
        //     .context("the given `out` node does not exists")?
        //     .outbound
        //     .entry(port_out.port_label())
        //     .or_default()
        //     .insert(port_in.clone());

        // self.vertices
        //     .get_mut(&vertex_in)
        //     .context("the given `in` node does not exists")?
        //     .inbound
        //     .insert(port_in.port_label(), port_out);

        Ok(())
    }

    // pub fn vertex_for(&self, vertex_id: VertexId) -> Result<Vertex, anyhow::Error> {
    //     self.vertices
    //         .get(&vertex_id)
    //         .cloned()
    //         .context("no node was found at the given id")
    // }

    // pub fn node_handle_for(&self, node_id: NodeId) -> Result<NodeHandle, anyhow::Error> {
    //     NodeHandle::new(node_id)
    //     self.vertex_for(VertexId::Node(node_id))?.node_handle()
    // }

    // pub fn function_handle_for(
    //     &self,
    //     function_id: FunctionId,
    // ) -> Result<FunctionHandle, anyhow::Error> {
    //     self.vertex_for(VertexId::Function(function_id))?
    //         .function_handle()
    // }

    pub fn main_function_id(&self) -> FunctionId {
        // SAFETY: unwrap is used because the main function must be inserted into the graph during Self::new()
        self.main_function_id
            .expect("`main_function_id` must be set in Graph::new()")
    }

    pub fn main_function_mut(&mut self) -> &mut Function {
        self.functions
            .get_mut(&self.main_function_id())
            .expect("`main function could not be found`")
    }

    // pub fn main_function_vertex(&self) -> Vertex {
    //     // SAFETY: unwrap is used because the main function must be inserted into the graph during Self::new()
    //     self.vertex_for(self.main_function_id().into())
    //         .expect("a main function must be inserted into the Graph")
    // }

    // pub fn fold_for(&self, port_label: impl Into<PortLabel>) -> Result<DataBox, anyhow::Error> {

    //     dbg!(main.inbound);

    //     unimplemented!()
    // }
}
