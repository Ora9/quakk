use std::{any::Any, collections::HashMap, str::FromStr};
use strum::IntoEnumIterator;

use crate::{GraphInoutId, Meta, Node, NodeInoutId};

#[derive(Debug, PartialEq, Eq, Hash)]
enum NumberInout {
    Output
}

#[derive(Debug, Default)]
pub struct Number {
    value: f32,
}

impl Node for Number {
    fn new() -> Self {
        Self::default()
    }

    fn id_for(&self, inout_name: &str) -> Option<NodeInoutId> {
        match inout_name {
            "out" => Some(NodeInoutId::new(inout_name)),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "Number"
    }

    fn evaluate(&self, output_id: Option<GraphInoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());

        dbg!(output_id);
        dbg!(meta);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum MultiplyInout {
    Term1,
    Term2,
    Out,
}

#[derive(Debug)]
pub struct Multiply();

impl Node for Multiply {
    fn new() -> Self {
        Self()
    }

    fn evaluate(&self, output_id: Option<GraphInoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());
    }

    fn title(&self) -> &str {
        "Multiply"
    }

    fn id_for(&self, inout_name: &str) -> Option<NodeInoutId> {
        match inout_name {
            "term1" | "term2" | "out" => Some(NodeInoutId::new(inout_name)),
            _ => None,
        }
    }
}
