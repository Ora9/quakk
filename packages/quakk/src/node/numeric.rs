use std::ops::{Add as opsAdd, Mul as opsMul};

use crate::{
    LasyFold, Meta, Node,
    id::{InId, InoutId, NodeId, NodeInId, NodeOutId, OutId},
};

#[derive(Debug, Default)]
pub struct Number {
    value: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberOutId {
    Out,
}

impl OutId for NumberOutId {}

impl Node for Number {
    fn new() -> Self {
        Self { value: 4.0 }
    }

    fn title(&self) -> &str {
        "Number"
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        if let Some(out_id) = out_id.as_any().downcast_ref::<NumberOutId>() {
            Some(NodeOutId::new(node_id, out_id))
        } else {
            None
        }
    }

    fn fold(&self, _out_id: &dyn OutId, _lasy_fold: LasyFold, _meta: Meta) -> anyhow::Result<f32> {
        Ok(self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiplyInId {
    Term1,
    Term2,
}

impl InId for MultiplyInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiplyOutId {
    Out,
}

impl OutId for MultiplyOutId {}

#[derive(Debug)]
pub struct Multiply;

impl Node for Multiply {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> &str {
        "Multiply"
    }

    fn fold(&self, _out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32> {
        let term1 = lasy_fold.get_in(&MultiplyInId::Term1, meta)?;
        let term2 = lasy_fold.get_in(&MultiplyInId::Term2, meta)?;

        Ok(term1.mul(term2))
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        if let Some(in_id) = in_id.as_any().downcast_ref::<MultiplyInId>() {
            Some(NodeInId::new(node_id, in_id))
        } else {
            None
        }
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        if let Some(out_id) = out_id.as_any().downcast_ref::<MultiplyOutId>() {
            Some(NodeOutId::new(node_id, out_id))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Add;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddInId {
    Term1,
    Term2,
}

impl InId for AddInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AddOutId {
    Out,
}

impl OutId for AddOutId {}

impl Node for Add {
    fn new() -> Self {
        Self
    }

    fn title(&self) -> &str {
        "Add"
    }

    fn fold(&self, _out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32> {
        let term1 = lasy_fold.get_in(&AddInId::Term1, meta)?;
        let term2 = lasy_fold.get_in(&AddInId::Term2, meta)?;

        Ok(term1.add(term2))
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        if let Some(in_id) = in_id.as_any().downcast_ref::<AddInId>() {
            Some(NodeInId::new(node_id, in_id))
        } else {
            None
        }
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        if let Some(out_id) = out_id.as_any().downcast_ref::<AddOutId>() {
            Some(NodeOutId::new(node_id, out_id))
        } else {
            None
        }
    }
}
