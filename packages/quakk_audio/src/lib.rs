use std::ops::{Add as opsAdd, Mul as opsMul};

use quakk::{InoutId, LasyFold, Meta, Node, OutId};

// #[derive(Debug, PartialEq, Eq, Hash)]
// enum NumberInout {
//     Output,
// }

#[derive(Debug, Default)]
pub struct LFO {
    frequency: f32,
    phase: f32,
}

impl Node for LFO {
    fn new() -> Self {
        Self::default()
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId> {
        match inout_name {
            "out" => Some(InoutId::new_out_from("out")),
            "frequency" | "phase" => Some(InoutId::new_in_from(inout_name)),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        "LFO"
    }

    fn fold(&self, out_id: OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32> {
        Ok((meta.tick as f32).mul(self.frequency).add(self.phase))
    }
}
