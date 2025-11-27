use std::any::Any;

use crate::{InoutId, Meta, Node};

#[derive(Debug, Hash)]
pub enum OscEdgepoints {
    Frequency,
    Amplitude,
    Out,
}

#[derive(Debug)]
pub struct Osc;

impl Node for Osc {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> &str {
        "Osc"
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);

    }
}
