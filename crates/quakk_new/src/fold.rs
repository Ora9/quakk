use std::{rc::Rc, sync::Mutex};

use crate::{DataBox, Graph, PortLabel, VertexId};

pub struct LasyFold {
    vertex_id: VertexId,
    graph: Rc<Mutex<Graph>>,
}

impl LasyFold {
    pub fn new(vertex_id: VertexId, graph: Rc<Mutex<Graph>>) -> Self {
        Self { vertex_id, graph }
    }

    pub fn get_in(&self, port_label: impl Into<PortLabel>) -> Result<DataBox, anyhow::Error> {
        let graph = self
            .graph
            .lock()
            .expect("the graph has been poisoned, who was it!?");

        dbg!(graph.vertex_for(self.vertex_id));

        unimplemented!()
    }
}
