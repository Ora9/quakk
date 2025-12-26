use std::{any::Any, collections::HashMap, str::FromStr};

use crate::{HashId, InoutId, Meta, Node, NodeId};

#[derive(Debug, PartialEq, Eq, Hash)]
enum NumberInout {
    Output,
}

#[derive(Debug, Default)]
pub struct Number {
    value: f32,
}

impl Node for Number {
    fn new() -> Self {
        Self::default()
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "out" => Some(InoutId::new_out_from("out")),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "Number"
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
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
pub struct Multiply;

impl Node for Multiply {
    fn new() -> Self {
        Self
    }

    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta) {
        dbg!(self.title());
    }

    fn title(&self) -> &str {
        "Multiply"
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
            "out" => Some(InoutId::new_in_from(inout_name)),
            _ => None,
        }
    }
}
