use std::{rc::Rc, sync::Mutex};

mod id;
pub use id::*;

mod data;
pub use data::*;

mod node;
pub use node::*;

mod graph;
pub use graph::*;

pub struct Quakk {
    pub graph: Rc<Mutex<Graph>>,
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
}
