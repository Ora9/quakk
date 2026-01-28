use std::fmt::Debug;

use crate::{
    LasyFold, Meta, NodeId, OutId,
    id::{InId, InoutId, NodeInId, NodeInoutId, NodeOutId},
};

pub mod numeric;
pub mod textual;

pub trait Node: Debug {
    fn new() -> Self
    where
        Self: Sized;

    /// The node "title" when displayed
    fn title(&self) -> &str;

    fn fold(&self, out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<f32>;

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId>;
    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId>;

    // fn node_inout_id_for(&self, inout_name: &str, node_id: NodeId) -> Option<NodeInoutId> {
    //     self.id_for(inout_name)
    //         .and_then(|inout_id| Some(NodeInoutId::new(node_id, inout_id)))
    // }

    // fn id_for(&self, inout_name: &str) -> Option<InoutId>;

    // fn in_id_for(&self, in_name: &str) -> Option<InId> {
    //     self.id_for(in_name)
    //         .and_then(|inout_id| inout_id.try_into().ok())
    // }

    // fn out_id_for(&self, out_name: &str) -> Option<OutId> {
    //     self.id_for(out_name)
    //         .and_then(|inout_id| inout_id.try_into().ok())
    // }
}
