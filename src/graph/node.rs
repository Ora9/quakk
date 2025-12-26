use std::{any::Any, fmt::Debug};

use crate::{HashId, InoutId, Meta, NodeId, NodeInoutId};

// pub mod audio;
pub mod numeric;
pub use numeric::*;
// pub mod textual;

// pub use textual::*;

pub trait Node: Debug {
    fn title(&self) -> &str;
    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta);

    fn node_inout_id_for(&self, inout_name: &str, node_id: NodeId) -> Option<NodeInoutId> {
        self.id_for(inout_name).and_then(|inout_id| {
            Some(NodeInoutId::new(node_id, inout_id))
        })
    }

    fn id_for(&self, inout_name: &str) -> Option<InoutId>;

    fn new() -> Self
    where
        Self: Sized;
}

// #[derive(Debug, PartialEq, Eq, Hash)]
// enum WeightedAverageInout {
//     Output,
//     InWeigth(usize),
//     InData(usize),
// }

// #[derive(Debug)]
// pub struct WeightedAverage {
//     // inout_ids: HashMap<NumericValueInout, NodeInoutId>,
// }

// impl Node for WeightedAverage {
//     fn new() -> Self {
//         // let mut inout_ids = HashMap::new();

//         // for inout in NumericValueInout::iter() {
//         //     dbg!(&inout);
//         //     inout_ids.insert(inout, NodeInoutId::new());
//         // };

//         Self {
//             // inout_ids: inout_ids,
//         }
//     }

//     fn id_for(&self, inout_name: &str) -> Option<GraphInoutId> {
//     //     if let Ok(inout_enum) = NumericValueInout::from_str(inout_name) {
//     //         self.inout_ids.get(&inout_enum).cloned()
//     //     } else {
//             None
//         // }
//     }

//     fn title(&self) -> &str {
//         "Weighted Average"
//     }

//     fn evaluate(&self, output_id: Option<GraphInoutId>, input: Box<dyn Any>, meta: Meta) {
//         dbg!(self.title());

//         dbg!(output_id);
//         dbg!(meta);
//     }
// }
