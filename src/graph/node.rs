use std::{any::Any, fmt::Debug};

use crate::{EdgepointId, Meta};

pub mod audio;
pub mod numeric;
pub mod textual;

pub use textual::*;

pub trait Node: Debug {
    fn title(&self) -> &str;
    fn evaluate(&self, output_id: Option<EdgepointId>, input: Box<dyn Any>, meta: Meta);
    fn new() -> Self where Self: Sized;
}
