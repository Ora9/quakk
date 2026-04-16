use std::hash::{BuildHasher, Hasher, RandomState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexId {
    Node(NodeId),
    Function(FunctionId),
}

impl VertexId {
    pub fn from_port_id(port_id: &PortId) -> Self {
        match port_id {
            PortId::Node(node_id) => Self::Node(node_id.node_id),
            PortId::Function(function_id) => Self::Function(function_id.function_id),
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

/// Identifies a [`Node`]
///
/// Each new node inserted in a [`Graph`] is assigned a new random id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    id: u64,
}

impl NodeId {
    /// Return a new random `NodeId`
    ///
    /// ```
    /// # use quakk::NodeId;
    ///
    /// assert_ne!(NodeId::new_random(), NodeId::new_random());
    /// ```
    pub fn new_random() -> Self {
        Self {
            id: RandomState::new().build_hasher().finish(),
        }
    }
}

/// Identifies a [`Function`]
///
/// Each new function declared in a [`Graph `] is assigned a new random id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: u64,
}

impl FunctionId {
    /// Return a new random `FunctionId`
    /// ```
    /// # use quakk::FunctionId;
    ///
    /// assert_ne!(FunctionId::new_random(), FunctionId::new_random());
    /// ```
    pub fn new_random() -> Self {
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

    fn direction(&self) -> PortDirection;
}

// impl<T> From<T> for Por
// where
//     T: Port,
// {
//     fn from(value: PortLabel) -> PortLabel {}
// }

/// Point to either a node port or function port
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortId {
    Node(NodePortId),
    Function(FunctionPortId),
}

impl PortId {
    pub fn port_label(&self) -> PortLabel {
        match self {
            Self::Function(port_id) => port_id.label.clone(),
            Self::Node(port_id) => port_id.label.clone(),
        }
    }

    pub fn node_id(&self) -> Option<NodeId> {
        match self {
            Self::Node(node_port_id) => Some(node_port_id.node_id),
            _ => None,
        }
    }

    pub fn function_id(&self) -> Option<FunctionId> {
        match self {
            Self::Function(function_port_id) => Some(function_port_id.function_id),
            _ => None,
        }
    }

    pub fn as_vertex_id(&self) -> VertexId {
        match self {
            Self::Node(node_port_id) => VertexId::Node(node_port_id.node_id),
            Self::Function(function_port_id) => VertexId::Function(function_port_id.function_id),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodePortId {
    node_id: NodeId,
    label: PortLabel,
}

impl NodePortId {
    pub fn new(node_id: NodeId, label: PortLabel) -> Self {
        Self { node_id, label }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionPortId {
    function_id: FunctionId,
    label: PortLabel,
}

impl FunctionPortId {
    pub fn new(function_id: FunctionId, label: PortLabel) -> Self {
        Self { function_id, label }
    }
}
