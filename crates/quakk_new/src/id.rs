use std::hash::{BuildHasher, Hasher, RandomState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexId {
    Node(NodeId),
    Function(FunctionId),
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

#[derive(Debug)]
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
        PortLabel::new(Self::to_str(&self))
    }

    fn direction(&self) -> PortDirection;
}

// impl<T> From<T> for Por
// where
//     T: Port,
// {
//     fn from(value: PortLabel) -> PortLabel {}
// }

#[derive(Debug)]
pub enum PortId {
    Function(FunctionPortId),
    Node(NodePortId),
}

#[derive(Debug)]
pub struct NodePortId {
    node_id: NodeId,
    port_label: PortLabel,
}

#[derive(Debug)]
pub struct FunctionPortId {
    function_id: FunctionId,
    port_label: PortLabel,
}
