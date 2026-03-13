use std::sync::{Arc, Mutex};

use anyhow::Context;

use crate::{
    Data, Graph, Meta,
    id::{InId, InoutId, NodeId},
};

/// `LasyFold` [folds] the [`Graph`] into a single value.
///
/// It can be viewed a the "evaluator" or "executor" of Quakk, the term "fold" is used to tie back
/// to the functional programming.
///
/// The fold is "lasy" because it will not blindly fold every node in the graph, but only compute
/// input that are asked for by each nodes. This is because every input of a node are not always
/// required to compute some output
///
/// The inner working of `LasyExecutor` is subject to change, to allow more flexibility, and performance.
///
/// `LasyFold` recursively traverse a [`Graph`] and any contained subgraphs, folding all needed nodes,
/// handling their ins and outs.
///
/// Currently :
/// - Recursively traverse the graph
/// - The struct is cheaply cloned.
/// - Each instance is specific to a node
/// - Holds the [`NodeId`] of the node, and a reference of the [`Graph`]
/// - Calls [`Node::fold()`][crate::Node::fold()], passing a new instance of `LasyFold`
/// - Node can call [`LasyFold::get_input()`] if they require some of their input for computing some
///   of their output
///
/// [folds]: https://en.wikipedia.org/wiki/Fold_(higher-order_function)
#[derive(Debug, Clone)]
pub struct LasyFold {
    node_id: NodeId,
    graph: Arc<Mutex<Graph>>,
}

impl LasyFold {
    /// Create a new `LasyFold`
    pub fn new(node_id: NodeId, graph: Arc<Mutex<Graph>>) -> Self {
        Self { node_id, graph }
    }

    pub fn get_in(&self, in_id: &dyn InId, meta: Meta) -> anyhow::Result<Data> {
        let (in_node_handle, in_node_out_id) = {
            let graph = self
                .graph
                .lock()
                .expect("the graph has been poisoned, who was it!?");

            let in_node_out_id = graph
                .vertex_for_id(self.node_id)
                .context("Should be able to find the node associated with this LasyInput")?
                .inbound_for(in_id)
                .context(format!(
                    "The node does not have any inbound edge for InId `{:?}`",
                    in_id
                ))?
                .to_owned();

            let in_node_handle = graph
                .handle_for_id(in_node_out_id.node_id())
                .context("Could not find the the `OutId` associated with this edge")?;

            (in_node_handle, in_node_out_id)
        };

        dbg!(in_node_handle.node().title());

        in_node_handle.node().fold(
            &*in_node_out_id.out_id(),
            LasyFold::new(in_node_handle.node_id(), self.graph.clone()),
            meta,
        )
    }
}
