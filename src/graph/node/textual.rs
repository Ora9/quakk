use std::any::Any;

use crate::{EdgepointId, Meta, Node};

#[derive(Debug, Hash)]
pub enum StringValueEdgepoints {
    Output,
}

#[derive(Default, Debug)]
pub struct StringValue {
    value: String,
}

impl Node for StringValue {
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "String Value"
    }

    fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);

    }
}

#[derive(Debug, Hash)]
pub enum TrimStringEdgepoints {
    Input,
    Output,
}

#[derive(Debug)]
pub struct TrimString;

impl Node for TrimString {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> &str {
        "Trim String"
    }

    fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta) {

    }
}
