mod graph;
pub use graph::*;

mod lasy;
pub use lasy::*;

mod node;
pub use node::Node;
pub use node::numeric;

mod meta;
pub use meta::*;

pub mod id;

use anyhow::{Context, anyhow};
use std::sync::{Arc, Mutex};

use crate::id::InId;
use crate::id::{NodeId, OutId};

#[derive(Debug)]
pub struct Quakk {
    pub graph: Arc<Mutex<Graph>>,
    pub base_meta: Meta,
}

impl Quakk {
    pub fn new() -> Self {
        let graph = Arc::new(Mutex::new(Graph::new()));

        Self {
            base_meta: Meta {
                quality: Quality::Balanced,
                tick: 0,
            },

            graph,
        }
    }

    pub fn fold_for(&self, graph_out_out_id: GraphOutOutId) -> Result<f32, anyhow::Error> {
        let graph_out_out_id: &dyn OutId = &graph_out_out_id;

        let graph_out_handle = {
            self.graph
                .lock()
                .expect("The graph has beend poisoned, who was it ?!")
                .graph_out_handle()
        };

        graph_out_handle
            .node()
            .fold(
                graph_out_out_id,
                LasyFold::new(NodeId::GraphOut, self.graph.clone()),
                self.base_meta,
            )
            .context("Could not evaluate the graph")
    }
}
