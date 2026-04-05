use std::fmt::Debug;

use anyhow::Context;

use crate::{Data, DataBox, PortLabel};

pub mod numeric;

#[derive(Debug)]
pub struct NodeBox {
    inner: Box<dyn Node>,
}

impl NodeBox {
    pub fn new(node: Box<dyn Node>) -> Self {
        NodeBox { inner: node }
    }

    pub fn mutate(
        mut self,
        port: impl Into<PortLabel>,
        value: impl Into<DataBox>,
    ) -> Result<Self, anyhow::Error> {
        self.inner
            .mutate(port.into(), value.into())
            .context("could not mutate")?;
        Ok(self)
    }
}

pub trait Node: Debug {
    fn init() -> NodeBox
    where
        Self: Sized;

    fn title(&self) -> &str;

    fn mutate(&mut self, port_label: PortLabel, value: DataBox) -> Result<(), anyhow::Error>;

    // fn fold(&mut self, port_out: PortLabel) -> DataBox;
}
