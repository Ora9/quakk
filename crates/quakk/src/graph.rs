use std::{collections::HashMap, rc::Rc, sync::Mutex};

use anyhow::Context;
use thiserror::Error;

use crate::{FunctionId, Node, NodeId, PortId};

mod function;
pub use function::*;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum GraphError {
    #[error("could not patch")]
    PatchError(#[from] FunctionPatchError),

    #[error("function `{0}` not found")]
    FunctionNotFound(FunctionId),
}

/// An edge between ports, either a node or function port
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    source: PortId,
    target: PortId,
}

impl Edge {
    /// New edge between to ports
    fn new(source: PortId, target: PortId) -> Self {
        Self { source, target }
    }

    /// Return the source `PortId` of the edge
    pub fn source(&self) -> &PortId {
        &self.source
    }

    /// Return the target `PortId` of the edge
    pub fn target(&self) -> &PortId {
        &self.target
    }
}

/// A `Graph` hold functions and nodes, it handle patches (connection between nodes)
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

        let main = FunctionDef {
            name: "Main".to_string(),
            color: 256,
        };
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
            eprintln!(
                "failed attempt to create a new id the graph (id already exists), retrying.."
            );
            self.next_function_id()
        }
    }
}

/// # Insertion
impl Graph {
    /// Insert the given node in the main function
    pub fn insert_in_main(&mut self, node: Node) -> NodeId {
        let main_function_id = self.main_function_id();

        self.insert_in(main_function_id, node)
    }

    /// Insert the given node into the specified function, or create the function with a default
    /// name if it does not exists
    pub fn insert_in(&mut self, function_id: FunctionId, node: Node) -> NodeId {
        let function = self
            .functions
            .entry(function_id)
            .or_insert(Function::new(FunctionDef::default_for(function_id)));

        let node_id = function.next_node_id(function_id);

        function.insert_node(node_id, node);

        node_id
    }

    /// Create a new function based on the given [`FunctionDef`], and return its id
    pub fn insert_function(&mut self, function_def: FunctionDef) -> FunctionId {
        let function_id = self.next_function_id();

        self.functions
            .insert(function_id, Function::new(function_def));

        function_id
    }
}

/// # Patching
impl Graph {
    /// Patch two ports
    pub fn patch(
        &mut self,
        source: impl Into<PortId>,
        target: impl Into<PortId>,
    ) -> Result<(), GraphError> {
        let source: PortId = source.into();
        let target: PortId = target.into();

        let function = self
            .functions
            .get_mut(&source.function_id())
            .ok_or(GraphError::FunctionNotFound(source.function_id()))?;

        function.patch(source, target).map_err(|err| err.into())
    }

    fn find_edge(&self, port_id: &PortId, predicate: impl Fn(&Edge) -> bool) -> Option<&Edge> {
        // TODO: lol that's kinda ugly that we call our predicate in another anonymous closure,
        // can't we directly take a closure that would be use by find() ?
        self.functions
            .get(&port_id.function_id())?
            .edges()
            .iter()
            .find(|&edge| predicate(edge))
    }

    pub fn function_for_id(&self, function_id: FunctionId) -> Option<&Function> {
        self.functions.get(&function_id)
    }

    pub fn node_for_id(&self, node_id: NodeId) -> Option<&Rc<Mutex<Node>>> {
        let function = self.function_for_id(node_id.function_id())?;
        function.node_for_id(node_id)
    }

    pub fn edge_for_port(&self, port_id: PortId) -> Option<&Edge> {
        self.find_edge(&port_id, |edge| {
            *edge.source() == port_id || *edge.target() == port_id
        })
    }

    pub fn edge_for_source_port(&self, port_id: PortId) -> Option<&Edge> {
        self.find_edge(&port_id, |edge| *edge.source() == port_id)
    }

    pub fn edge_for_target_port(&self, port_id: PortId) -> Option<&Edge> {
        self.find_edge(&port_id, |edge| *edge.target() == port_id)
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

    pub fn functions(&self) -> &HashMap<FunctionId, Function> {
        &self.functions
    }
}
