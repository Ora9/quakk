use std::ops::{Add as opsAdd, Mul as opsMul};

use quakk::{
    Data, LasyFold, Meta, Node,
    id::{InId, InoutId, NodeId, NodeInId, NodeOutId, OutId},
};

#[derive(Debug, Default)]
pub struct LFO {
    frequency: f32,
    phase: f32,
}

impl LFO {
    fn new() -> Self {
        Self::default()
    }
}

impl Node for LFO {
    fn initialize() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "LFO"
    }

    fn fold(&self, out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<Data> {
        Ok(Data::new(
            (meta.tick as f32).mul(self.frequency).add(self.phase),
        ))
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        None
    }
}
