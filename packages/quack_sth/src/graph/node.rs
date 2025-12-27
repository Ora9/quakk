use std::{fmt::Debug};

use crate::{InoutId, LasyFold, Meta, NodeId, NodeInoutId};

pub trait Node: Debug {
    fn new() -> Self
    where
        Self: Sized;

    /// The node "title" when displayed
    fn title(&self) -> &str;

    fn fold(&self, out_id: InoutId, lasy_fold: LasyFold, meta: Meta) -> f32;

    fn node_inout_id_for(&self, inout_name: &str, node_id: NodeId) -> Option<NodeInoutId> {
        self.id_for(inout_name)
            .and_then(|inout_id| Some(NodeInoutId::new(node_id, inout_id)))
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId>;
}
