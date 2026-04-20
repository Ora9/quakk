use std::{rc::Rc, sync::Mutex};

mod id;
pub use id::*;

mod data;
pub use data::*;

mod node;
pub use node::*;

mod graph;
pub use graph::*;

mod fold;
pub use fold::*;

pub struct Quakk {
    graph: Rc<Mutex<Graph>>,
}

impl Default for Quakk {
    fn default() -> Self {
        Self {
            graph: Rc::new(Mutex::new(Graph::new())),
        }
    }
}

impl Quakk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn graph<R>(&self, reader: impl FnOnce(&Graph) -> R) -> R {
        let graph = self
            .graph
            .lock()
            .expect("the graph has been poisoned, who was it ?!");

        reader(&graph)
    }

    pub fn graph_mut<R>(&mut self, writer: impl FnOnce(&mut Graph) -> R) -> R {
        let mut graph = self
            .graph
            .lock()
            .expect("the graph has been poisoned, who was it ?!");

        writer(&mut graph)
    }

    pub fn fold_for(&self, port_label: impl Into<PortLabel>) -> Result<Data, anyhow::Error> {
        let main_function = self.graph(|graph| graph.main_function_id());

        let lasy_fold = LasyFold::new(self.graph.clone(), main_function.into());
        lasy_fold.get_in(port_label)
    }
}
