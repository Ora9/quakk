use std::sync::{Arc, Mutex};

use anyhow::Context;

use crate::{Graph, InoutId, Meta, NodeId};

/// `LasyExecutor` is the executor of Quack, constructed with a reference to the graph, it is responsible for
/// evaluating all needed nodes, handling their ins and outs
///
/// The inner working of `LasyExecutor` is subject to change, to allow for more performance
///
/// Currently, `LasyExecutor` :
/// - Is cheaply cloned
/// - Recursively traverse the graph
#[derive(Debug, Clone)]
pub struct LasyExecutor {
    node_id: NodeId,
    graph: Arc<Mutex<Graph>>,
}

impl LasyExecutor {
    pub fn new(node_id: NodeId, graph: Arc<Mutex<Graph>>) -> Self {
        Self { node_id, graph }
    }

    pub fn get_from(&self, in_id: InoutId, meta: Meta) -> Option<f32> {
        dbg!(in_id);

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

        inbound_node.node().evaluate(
            inbound_out_id.inout_id(),
            LasyExecutor::new(inbound_node.node_id(), self.graph.clone()),
            meta,
        );

        None

        // node_handle.

        // node_handle.node().evaluate(, lasy_executor, meta);
    }
}
