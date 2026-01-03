use std::ops::{Add as opsAdd, Mul as opsMul};

use crate::{InoutId, LasyFold, Meta, Node, OutId};

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

    fn title(&self) -> &str {
        "Number"
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "out" => Some(InoutId::new_out_from("out")),
            _ => None,
        }
    }

    fn fold(&self, _out_id: OutId, _lasy_fold: LasyFold, _meta: Meta) -> anyhow::Result<f32> {
        Ok(self.value)
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

    fn title(&self) -> &str {
        "Multiply"
    }

    fn fold(&self, _out_id: OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32> {
        let term1 = lasy_fold.get_in(self.id_for("term1").unwrap().try_into()?, meta)?;
        let term2 = lasy_fold.get_in(self.id_for("term2").unwrap().try_into()?, meta)?;

        Ok(term1.mul(term2))
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
            "out" => Some(InoutId::new_out_from(inout_name)),
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

    fn title(&self) -> &str {
        "Add"
    }

    fn fold(&self, _out_id: OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32> {
        let term1 = lasy_fold.get_in(self.id_for("term1").unwrap().try_into()?, meta)?;
        let term2 = lasy_fold.get_in(self.id_for("term2").unwrap().try_into()?, meta)?;

        Ok(term1.add(term2))
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
            "out" => Some(InoutId::new_out_from(inout_name)),
            _ => None,
        }
    }
}
