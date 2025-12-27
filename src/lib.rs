#![allow(dead_code)]
#![allow(unused)]

mod graph;
use std::sync::{Arc, Mutex};

pub use graph::*;

mod lasy;
pub use lasy::*;

#[derive(Debug)]
pub struct Quack {
    pub graph: Arc<Mutex<Graph>>,
    pub lasy_executor: LasyExecutor,
    pub base_meta: Meta,
}

impl Quack {
    pub fn new() -> Self {
        let graph = Arc::new(Mutex::new(Graph::new()));

        Self {
            lasy_executor: LasyExecutor::new(graph.clone()),
            base_meta: Meta {
                quality: Quality::Balanced,
                tick: 0,
            },

            graph,
        }
    }

    pub fn evaluate_for(&self, inout_name: &str) {
        self.lasy_executor.evaluate_for(inout_name, self.base_meta);
    }
}
