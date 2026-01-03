use anyhow::{Context, anyhow};
use std::sync::{Arc, Mutex};

mod graph;
pub use graph::*;

mod lasy;
pub use lasy::*;

mod node;
pub use node::Node;
pub use node::numeric;

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

    pub fn evaluate_for(&self, out_name: &str) -> Result<f32, anyhow::Error> {
        LasyFold::new(NodeId::GraphOut, self.graph.clone())
            .get_in(InId::new(out_name), self.base_meta)
            .context("Could not evaluate the graph")

        // let out_id = {
        //     self.graph
        //         .lock()
        //         .expect("the graph has been poisoned, who was it ?!")
        //         .graph_out_id_for(out_name)
        //         .context("out name not found for this graph")?
        // };

        // let lasy_fold = LasyFold::new(out_id.node_id(), self.graph.clone());

        // lasy_fold
        //     .get_input(out_id.inout_id(), self.base_meta)
        //     .ok_or(anyhow!("prout"))
    }
}
