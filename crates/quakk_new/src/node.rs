use std::{fmt::Debug, rc::Rc};

use anyhow::Context;

use crate::{Data, DataTrait, LasyFold, NodeId, NodePortId, PortId, PortLabel};

pub mod numeric;

#[derive(Debug)]
pub struct NodeHandle {
    id: NodeId,
    node: Rc<NodeBox>,
}

impl Clone for NodeHandle {
    fn clone(&self) -> Self {
        NodeHandle {
            id: self.id,
            node: self.node.clone(),
        }
    }
}

impl NodeHandle {
    pub(crate) fn new(node_id: NodeId, node: NodeBox) -> Self {
        Self {
            id: node_id,
            node: Rc::new(node),
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.id
    }

    pub fn node(&self) -> Rc<NodeBox> {
        self.node.clone()
    }

    pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
        PortId::Node(NodePortId::new(self.id, label.into()))
    }

    pub fn out(&self) -> PortId {
        self.port_id("out")
    }

    pub fn r#in(&self) -> PortId {
        self.port_id("in")
    }
}

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
