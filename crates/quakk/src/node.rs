use std::fmt::Debug;

use anyhow::Context;

use crate::{Data, LasyFold, PortLabel};

pub mod numeric;
pub mod text;

#[derive(Debug)]
pub struct Node {
    inner: Box<dyn NodeTrait>,
}

impl Node {
    pub fn new(node: Box<dyn NodeTrait>) -> Self {
        Node { inner: node }
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

pub trait NodeTrait: Debug {
    fn init() -> Node
    where
        Self: Sized;

    fn title(&self) -> &str;

    fn mutate(&mut self, port: PortLabel, value: Data) -> Result<(), anyhow::Error>;

    fn fold(&mut self, port: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error>;
}
