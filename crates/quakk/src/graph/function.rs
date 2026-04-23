use std::{
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
    sync::Mutex,
};

use crate::{Edge, FunctionId, Node, NodeId, PortId};
use anyhow::anyhow;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FunctionPatchError {
    #[error(
        "a patch cannot be done between nodes of two differents functions, source id `{source}` != target id `{target}`"
    )]
    NotSameFunction {
        r#source: FunctionId,
        target: FunctionId,
    },
}

/// Function definition passed to `Function::new()`
#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub color: u32,
}

impl FunctionDef {
    /// Generate a `FunctionDef` based on a given `FunctionId` ()
    pub(super) fn default_for(function_id: FunctionId) -> Self {
        FunctionDef {
            name: format!("Function-{}", function_id.as_u64()),
            color: 0,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    def: FunctionDef,
    nodes: HashMap<NodeId, Rc<Mutex<Node>>>,
    edges: HashSet<Edge>,
    last_node_id: Option<NodeId>,
}

impl Function {
    pub(super) fn new(def: FunctionDef) -> Self {
        Self {
            def,
            nodes: HashMap::new(),
            edges: HashSet::new(),

            last_node_id: None,
        }
    }

    pub(super) fn next_node_id(&mut self, function_id: FunctionId) -> NodeId {
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
                "failed attempt to create a new id in function #{:?} (id already exists), retrying..",
                function_id
            );
            self.next_node_id(function_id)
        }
    }

    pub(super) fn insert_node(&mut self, node_id: NodeId, node: Node) {
        self.nodes.insert(node_id, Rc::new(Mutex::new(node)));
    }

    pub(super) fn patch(
        &mut self,
        source: PortId,
        target: PortId,
    ) -> Result<(), FunctionPatchError> {
        // TODO: should make sure that the port_id we are given is in our function

        if source.function_id() != target.function_id() {
            Err(FunctionPatchError::NotSameFunction {
                source: source.function_id(),
                target: target.function_id(),
            })
        } else {
            self.edges.insert(Edge::new(source, target));

            Ok(())
        }
    }

    pub(super) fn node_for_id(&self, node_id: NodeId) -> Option<&Rc<Mutex<Node>>> {
        self.nodes().get(&node_id)
    }

    pub(super) fn nodes(&self) -> &HashMap<NodeId, Rc<Mutex<Node>>> {
        &self.nodes
    }

    pub(super) fn edges(&self) -> &HashSet<Edge> {
        &self.edges
    }
}
