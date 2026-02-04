use std::ops::{Add as opsAdd, Div, Mul as opsMul, Sub};

use anyhow::Context;

use crate::{
    Data, LasyFold, Meta, Node,
    id::{InId, InoutId, NodeId, NodeInId, NodeOutId, OutId},
};

#[derive(Debug, Default)]
pub struct NumericConstant {
    value: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumericConstantOutId {
    Out,
}

impl OutId for NumericConstantOutId {}

impl NumericConstant {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Node for NumericConstant {
    fn initialize() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "Numeric Constant"
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        out_id
            .as_any()
            .downcast_ref::<NumericConstantOutId>()
            .map(|out_id| NodeOutId::new(node_id, out_id))
    }

    fn fold(&self, _out_id: &dyn OutId, _lasy_fold: LasyFold, _meta: Meta) -> anyhow::Result<Data> {
        Ok(Data::new(self.value))
    }
}

#[derive(Debug, Default)]
pub enum ArithmeticOperation {
    #[default]
    Addition,
    Substraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArithmeticsInId {
    Term1,
    Term2,
}

impl InId for ArithmeticsInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ArithmeticsOutId {
    Out,
}

impl OutId for ArithmeticsOutId {}

#[derive(Debug, Default)]
pub struct Arithmetics {
    operation: ArithmeticOperation,
}

impl Arithmetics {
    pub fn new(operation: ArithmeticOperation) -> Self {
        Self { operation }
    }
}

impl Node for Arithmetics {
    fn initialize() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "Arithmetics"
    }

    fn fold(&self, _out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<Data> {
        let term1 = lasy_fold
            .get_in(&ArithmeticsInId::Term1, meta)?
            .into_f32()
            .context("type mismatch for Term1")?;
        let term2 = lasy_fold
            .get_in(&ArithmeticsInId::Term2, meta)?
            .into_f32()
            .context("type mismatch for Term2")?;

        use ArithmeticOperation::*;
        let res = match self.operation {
            Addition => term1.add(term2),
            Substraction => term1.sub(term2),
            Multiplication => term1.mul(term2),
            Division => term1.div(term2),
        };

        Ok(Data::new(res))
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        in_id
            .as_any()
            .downcast_ref::<ArithmeticsInId>()
            .map(|in_id| NodeInId::new(node_id, in_id))
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        out_id
            .as_any()
            .downcast_ref::<ArithmeticsOutId>()
            .map(|out_id| NodeOutId::new(node_id, out_id))
    }
}
