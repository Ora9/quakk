use std::sync::{Arc, Mutex};

use crate::{Graph, InoutId, Meta, NodeId};

/// `LasyFold` [folds] the [`Graph`] into a single value.
///
/// It can be viewed a the "evaluator" or "executor" of Quack, the term "fold" is used to tie back
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

    pub fn get_input(&self, in_id: InoutId, meta: Meta) -> Option<f32> {
        let (inbound_node, inbound_out_id) = {
            let graph = self.graph.lock().unwrap();
            let inbound_out_id = graph
                .vertex_for_id(self.node_id)
                .expect("Cannot find the given node")
                .inbound_for(in_id)?
                .to_owned();

            (
                graph.handle_for_id(inbound_out_id.node_id())?,
                inbound_out_id,
            )
        };

        dbg!(inbound_node.node().title());

        Some(inbound_node.node().fold(
            inbound_out_id.inout_id(),
            LasyFold::new(inbound_node.node_id(), self.graph.clone()),
            meta,
        ))
    }
}
