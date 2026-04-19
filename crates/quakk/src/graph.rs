use std::collections::{HashMap, HashSet};

use anyhow::{Context, Ok, bail};

use crate::{FunctionId, Node, NodeId, PortId};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    source: PortId,
    target: PortId,
}

impl Edge {
    fn new(source: PortId, target: PortId) -> Self {
        Self { source, target }
    }
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub color: u32,
}

impl FunctionDef {
    fn default_for(function_id: FunctionId) -> Self {
        FunctionDef {
            name: format!("Function-{}", function_id.as_u64()),
            color: 0,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    def: FunctionDef,
    nodes: HashMap<NodeId, Node>,
    edges: HashSet<Edge>,
    last_node_id: Option<NodeId>,
}

impl Function {
    pub fn new(def: FunctionDef) -> Self {
        Self {
            def,
            nodes: HashMap::new(),
            edges: HashSet::new(),

            last_node_id: None,
        }
    }

    fn default_for(function_id: FunctionId) -> Self {
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
        self.nodes.insert(node_id, node);
    }

    pub fn patch(&mut self, source: PortId, target: PortId) -> Result<(), anyhow::Error> {
        // TODO: should make sure that the port_id we are given is in our function
        if source.function_id() != target.function_id() {
            bail!("can't patch in two different function");
        }

        let edge = Edge::new(source, target);
        self.edges.insert(edge);

        Ok(())
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

        let main = Function::new(FunctionDef {
            name: "Main".to_string(),
            color: 256,
        });
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
            .or_insert(Function::new(FunctionDef::default_for(function_id)));

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
