use std::ops::{Add as opsAdd, Mul as opsMul};

use quack_sth::{InoutId, LasyFold, Meta, Node};

// #[derive(Debug, PartialEq, Eq, Hash)]
// enum NumberInout {
//     Output,
// }

#[derive(Debug, Default)]
pub struct Number {
    value: f32,
}

impl Node for Number {
    fn new() -> Self {
        Self { value: 4.0 }
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

    fn fold(&self, _out_id: InoutId, _lasy_fold: LasyFold, _meta: Meta) -> f32 {
        self.value
    }
}

// #[derive(Debug, PartialEq, Eq, Hash)]
// enum MultiplyInout {
//     Term1,
//     Term2,
//     Out,
// }

#[derive(Debug)]
pub struct Multiply;

impl Node for Multiply {
    fn new() -> Self {
        Self
    }

    fn fold(&self, _out_id: InoutId, lasy_fold: LasyFold, meta: Meta) -> f32 {
        if let Some(term1) = lasy_fold.get_input(self.id_for("term1").unwrap(), meta)
            && let Some(term2) = lasy_fold.get_input(self.id_for("term2").unwrap(), meta)
        {
            term1.mul(term2)
        } else {
            0.0
        }
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

#[derive(Debug)]
pub struct Add;

impl Node for Add {
    fn new() -> Self {
        Self
    }

    fn fold(&self, _out_id: InoutId, lasy_fold: LasyFold, meta: Meta) -> f32 {
        if let Some(term1) = lasy_fold.get_input(self.id_for("term1").unwrap(), meta)
            && let Some(term2) = lasy_fold.get_input(self.id_for("term2").unwrap(), meta)
        {
            term1.add(term2)
        } else {
            0.0
        }
    }

    fn title(&self) -> &str {
        "Add"
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
            "out" => Some(InoutId::new_in_from(inout_name)),
            _ => None,
        }
    }
}
