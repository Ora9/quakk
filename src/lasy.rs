use std::sync::{Arc, Mutex};

use crate::{Graph, Meta};

/// `LasyExecutor` is the executor of Quack, constructed with a reference to the graph, it is responsible for
/// evaluating all needed nodes, handling their ins and outs
///
/// The inner working of `LasyExecutor` is subject to change, to allow for more performance
///
/// Currently, `LasyExecutor` :
/// - Is cheaply cloned
/// - Recursively traverse the graph

#[derive(Debug)]
pub struct LasyExecutor {
    graph: Arc<Mutex<Graph>>,
}

impl LasyExecutor {
    pub fn new(graph: Arc<Mutex<Graph>>) -> Self {
        Self { graph }
    }

    pub(crate) fn evaluate_for(&self, out_name: &str, meta: Meta) {
        dbg!(out_name, meta);

        if let Ok(graph) = self.graph.lock() {
            dbg!(graph.graph_out_id_for(out_name));
        }
    }

    fn get() {

    }
}
