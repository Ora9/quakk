#![allow(dead_code)]
#![allow(unused)]

mod graph;
use std::sync::{Arc, Mutex};

use anyhow::Context;
pub use graph::*;

mod lasy;
pub use lasy::*;

#[derive(Debug)]
pub struct Quack {
    pub graph: Arc<Mutex<Graph>>,
    pub base_meta: Meta,
}

impl Quack {
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

    pub fn evaluate_for(&self, out_name: &str) -> Result<(), anyhow::Error> {
        // self.lasy_executor.evaluate_for(out_name, self.base_meta);
        let out_id = {
            self.graph
                .lock()
                .expect("the graph has been poisoned, who is it ?!")
                .graph_out_id_for(out_name)
                .context("out name not found for this graph")?
        };

        let lasy_executor = LasyExecutor::new(out_id.node_id(), self.graph.clone());
        lasy_executor.get_from(out_id.inout_id(), self.base_meta);

        Ok(())
    }
}
