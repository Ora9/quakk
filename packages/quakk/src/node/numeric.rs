use std::ops::{Add as opsAdd, Mul as opsMul};

use crate::{
    LasyFold, Meta, Node,
    id::{InId, InoutId, NodeId, NodeInId, NodeOutId, OutId},
};

// #[derive(Debug, PartialEq, Eq, Hash)]
// enum NumberInout {
//     Output,
// }

#[derive(Debug, Default)]
pub struct Number {
    value: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumberOutId {
    Out,
    Prout(u64),
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
            match out_id {
                NumberOutId::Out => Some(NodeOutId::new(node_id, out_id)),
                NumberOutId::Prout(prout) => Some(NodeOutId::new(node_id, out_id)),
                _ => None,
            }
        } else {
            None
        }
    }

    // fn id_for(&self, inout_name: &str) -> Option<InoutId> {
    //     match inout_name {
    //         "out" => Some(InoutId::new_out_from("out")),
    //         _ => None,
    //     }
    // }

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
        // let term1 = lasy_fold.get_in(Box::new(MultiplyInId::Term1), meta)?;
        // let term2 = lasy_fold.get_in(Box::new(MultiplyInId::Term2), meta)?;

        // Ok(term1.mul(term2))
        Ok(Default::default())
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        None
        // if let Some(out_id) = out_id.as_any().downcast_ref::<NumberOutId>() {
        //     match out_id {
        //         NumberOutId::Out => {
        //             dbg!(out_id);
        //             None
        //         }
        //         _ => None,
        //     }
        // } else {
        //     None
        // }
    }
    // fn id_for(&self, inout_name: &str) -> Option<InoutId> {
    //     match inout_name {
    //         "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
    //         "out" => Some(InoutId::new_out_from(inout_name)),
    //         _ => None,
    //     }
    // }
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
        // let term1 = lasy_fold.get_in(&AddInId::Term1.into(), meta)?;
        // let term2 = lasy_fold.get_in(&AddInId::Term2.into(), meta)?;

        // Ok(term1.add(term2))
        Ok(Default::default())
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        None
    }
    // fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
    //     in_id == AddInId::Term1;

    //     // match in_id {
    //     //     &AddInId::Term1 => Some(NodeInId::new(node_id, in_id)),
    //     //     &AddInId::Term1 => Some(NodeInId::new(node_id, in_id)),
    //     //     _ => None,
    //     // }
    //     None
    // }

    // fn id_for(&self, inout_name: &str) -> Option<InoutId> {
    //     match inout_name {
    //         "term1" | "term2" => Some(InoutId::new_in_from(inout_name)),
    //         "out" => Some(InoutId::new_out_from(inout_name)),
    //         _ => None,
    //     }
    // }
}
