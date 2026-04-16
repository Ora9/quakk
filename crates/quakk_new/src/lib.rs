use std::{rc::Rc, sync::Mutex};

mod id;
use anyhow::Context;
pub use id::*;

mod data;
pub use data::*;

mod function;
pub use function::*;

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
        let mut graph = self
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

    pub fn fold_for(&self, port_label: impl Into<PortLabel>) -> Result<DataBox, anyhow::Error> {
        let (entry_vertex, entry_out_port) = self.graph(|graph| {
            let main_function = graph.main_function_vertex();
            let entry_out_port = main_function
                .inbound_for(port_label)
                .context("no node is patched to this function port")?;

            let entry_vertex = graph
                .vertex_for(entry_out_port.as_vertex_id())
                .context("node should exist")?;

            Ok::<_, anyhow::Error>((entry_vertex, entry_out_port))
        })?;

        let node_handle = entry_vertex
            .node_handle()
            .context("function to function not yet handled")?;

        node_handle.node().fold(
            entry_out_port.port_label(),
            LasyFold::new(entry_out_port.as_vertex_id(), self.graph.clone()),
        );

        // main_function.

        unimplemented!()
    }
}
