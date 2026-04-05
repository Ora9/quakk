use std::hash::{BuildHasher, Hasher, RandomState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexId {
    Node(NodeId),
    Function(FunctionId),
}

impl VertexId {
    pub fn from_port_id(port_id: &PortId) -> Self {
        match port_id {
            PortId::Node(node_id) => Self::Node(node_id.id),
            PortId::Function(function_id) => Self::Function(function_id.id),
        }
    }
}

impl From<FunctionId> for VertexId {
    fn from(value: FunctionId) -> Self {
        Self::Function(value)
    }
}

impl From<NodeId> for VertexId {
    fn from(value: NodeId) -> Self {
        Self::Node(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: u64,
}

impl NodeId {
    pub fn new_random() -> Self {
        Self {
            id: RandomState::new().build_hasher().finish(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: u64,
}

impl FunctionId {
    pub fn new() -> Self {
        Self {
            id: RandomState::new().build_hasher().finish(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortLabel {
    label: String,
}

impl PortLabel {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.label
    }
}

impl From<&str> for PortLabel {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug)]
pub enum PortDirection {
    In,
    Out,
}

pub trait Port {
    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized;

    fn from_label(port_label: PortLabel) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_str(port_label.as_str())
    }

    fn to_str(&self) -> &str;

    fn to_label(&self) -> PortLabel {
        PortLabel::new(Self::to_str(self))
    }

    fn default_out() -> Option<PortLabel> {
        None
    }
    fn default_in() -> Option<PortLabel> {
        None
    }

    fn direction(&self) -> PortDirection;
}

// impl<T> From<T> for Por
// where
//     T: Port,
// {
//     fn from(value: PortLabel) -> PortLabel {}
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortId {
    Function(FunctionPortId),
    Node(NodePortId),
}

impl PortId {
    pub fn port_label(&self) -> PortLabel {
        match self {
            Self::Function(port_id) => port_id.label.clone(),
            Self::Node(port_id) => port_id.label.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodePortId {
    id: NodeId,
    label: PortLabel,
}

impl NodePortId {
    pub fn new(id: NodeId, label: PortLabel) -> Self {
        Self { id, label }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionPortId {
    id: FunctionId,
    label: PortLabel,
}

impl FunctionPortId {
    pub fn new(id: FunctionId, label: PortLabel) -> Self {
        Self { id, label }
    }
}
