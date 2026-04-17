use std::collections::{HashMap, HashSet};

use anyhow::{Context, Ok, bail};

use crate::{FunctionId, Node, NodeId, PortId, PortLabel};

/// Represent all inbound and outbound connection of either a node or a function
#[derive(Debug)]
struct Bounds {
    inbound: HashMap<PortLabel, PortId>,
    outbound: HashMap<PortLabel, HashSet<PortId>>,
}

impl Bounds {
    fn new() -> Self {
        Self {
            inbound: HashMap::new(),
            outbound: HashMap::new(),
        }
    }

    fn outbound_for(&self, port_label: impl Into<PortLabel>) -> Option<HashSet<PortId>> {
        self.outbound.get(&port_label.into()).cloned()
    }

    fn inbound_for(&self, port_label: impl Into<PortLabel>) -> Option<PortId> {
        self.inbound.get(&port_label.into()).cloned()
    }
}

#[derive(Debug)]
pub struct NodeBounds {
    node: Node,
    bounds: Bounds,
}

impl NodeBounds {
    fn new(node: Node) -> Self {
        Self {
            node,
            bounds: Bounds::new(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub color: u32,

    nodes: HashMap<NodeId, NodeBounds>,
    self_bounds: Bounds,
    last_node_id: Option<NodeId>,
}

impl Function {
    pub fn new(name: &str, color: u32) -> Self {
        Self {
            name: name.to_string(),
            color,

            nodes: HashMap::new(),
            self_bounds: Bounds::new(),

            last_node_id: None,
        }
    }

    fn default(function_id: FunctionId) -> Self {
        Self::new(&format!("Function-{}", function_id.as_u64()), 0)
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

    fn insert_node(&mut self, node_id: NodeId, node: Node) {
        self.nodes.insert(node_id, NodeBounds::new(node));
    }

    pub fn patch(&mut self, port_out: PortId, port_in: PortId) -> Result<(), anyhow::Error> {
        // dbg!(&port_out, &port_in);

        if port_out.function_id() != port_in.function_id() {
            bail!("can't patch in two different function");
        }

        // let mut bounds = |port_id| {
        //     if let PortId::Node(node_port_id) = port_id {
        //         Ok(&mut self
        //             .nodes
        //             .get_mut(&node_port_id.id())
        //             .context("could not find the given node in the function")?
        //             .bounds)
        //     } else {
        //         Ok(&mut self.self_bounds)
        //     }
        // };

        // let out_bound = bou

        // TODO: improve this :
        // - if early retrun (node not found, we can be in an inbetween state where only one of
        //   the bound is patched)
        // - too much repetition,

        {
            let out_bounds = if let PortId::Node(ref node_port_id) = port_out {
                &mut self
                    .nodes
                    .get_mut(&node_port_id.id())
                    .context("could not find the given node in the function")?
                    .bounds
            } else {
                &mut self.self_bounds
            };

            out_bounds
                .outbound
                .entry(port_out.port_label().clone())
                .or_default()
                .insert(port_in.clone());
        }

        {
            let in_bounds = if let PortId::Node(ref node_port_id) = port_in {
                &mut self
                    .nodes
                    .get_mut(&node_port_id.id())
                    .context("could not find the given node in the function")?
                    .bounds
            } else {
                &mut self.self_bounds
            };

            in_bounds
                .inbound
                .insert(port_in.port_label().clone(), port_out);
        }

        // .outbound
        // .entry(out_port_label);

        // let mut in_bounds = if let PortId::Node(node_port_id) = port_in {
        //     &self
        //         .nodes
        //         .get_mut(&node_port_id.id())
        //         .context("could not find the given node in the function")?
        //         .bounds
        // } else {
        //     &self.self_bounds
        // };

        // dbg!(out_bounds);

        Ok(())

        // let vertex_out = VertexId::from_port_id(&port_out);
        // let vertex_in = VertexId::from_port_id(&port_in);

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
    }
}

/// A `Graph` hold nodes and handle patches (connection between nodes)
#[derive(Debug)]
pub struct Graph {
    functions: HashMap<FunctionId, Function>,
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
    pub fn insert_in_main(&mut self, node: Node) -> NodeId {
        let main_function_id = self.main_function_id();

        self.insert_in(main_function_id, node)
    }

    /// Insert the given node into the specified function, or create the function if it does not exists
    pub fn insert_in(&mut self, function_id: FunctionId, node: Node) -> NodeId {
        let function = self
            .functions
            .entry(function_id)
            .or_insert(Function::default(function_id));

        let node_id = function.next_node_id(function_id);

        function.insert_node(node_id, node);

        node_id
    }

    pub fn insert_function(&mut self, function: Function) -> FunctionId {
        let function_id = self.next_function_id();

        self.functions.insert(function_id, function);

        function_id
    }

    pub fn patch(
        &mut self,
        port_out: impl Into<PortId>,
        port_in: impl Into<PortId>,
    ) -> Result<(), anyhow::Error> {
        let port_in: PortId = port_in.into();
        let port_out: PortId = port_out.into();

        let function = self
            .functions
            .get_mut(&port_out.function_id())
            .context("given nodes doesn't reside in an existing function")?;

        function.patch(port_out, port_in)
    }

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
}
