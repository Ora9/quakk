use std::{any::Any, fmt::Debug};

use crate::{InoutId, Meta};

pub mod audio;
pub mod numeric;
pub mod textual;

pub use textual::*;

pub trait Node: Debug {
    fn title(&self) -> &str;
    fn evaluate(&self, output_id: Option<InoutId>, input: Box<dyn Any>, meta: Meta);
    fn new() -> Self where Self: Sized;
}
