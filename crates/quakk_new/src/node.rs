use std::fmt::Debug;

use anyhow::Context;

use crate::{Data, LasyFold, PortLabel};

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
        value: impl Into<Data>,
    ) -> Result<Self, anyhow::Error> {
        self.inner
            .mutate(port.into(), value.into())
            .context("could not mutate")?;
        Ok(self)
    }

    pub fn fold(
        &mut self,
        port_out: PortLabel,
        lasy_fold: LasyFold,
    ) -> Result<Data, anyhow::Error> {
        self.inner.fold(port_out, lasy_fold)
    }
}

pub trait Node: Debug {
    fn init() -> NodeBox
    where
        Self: Sized;

    fn title(&self) -> &str;

    fn mutate(&mut self, port_label: PortLabel, value: Data) -> Result<(), anyhow::Error>;

    fn fold(&mut self, port_out: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error>;
}
