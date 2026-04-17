use std::fmt::Debug;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum VertexId {
//     Node(NodeId),
//     Function(FunctionId),
// }

// impl VertexId {
//     pub fn from_port_id(port_id: &VertexPortId) -> Self {
//         match port_id {
//             VertexPortId::Node(node_id) => Self::Node(node_id.node_id),
//             VertexPortId::Function(function_id) => Self::Function(function_id.function_id),
//         }
//     }

//     pub fn function_id(&self) -> Option<FunctionId> {
//         match self {
//             VertexId::Function(function_id) => Some(*function_id),
//             _ => None,
//         }
//     }

//     pub fn node_id(&self) -> Option<NodeId> {
//         match self {
//             VertexId::Node(node_id) => Some(*node_id),
//             _ => None,
//         }
//     }
// }

// impl From<FunctionId> for VertexId {
//     fn from(value: FunctionId) -> Self {
//         Self::Function(value)
//     }
// }

// impl From<NodeId> for VertexId {
//     fn from(value: NodeId) -> Self {
//         Self::Node(value)
//     }
// }

/// Identifies a [`Node`]
///
/// Each new node inserted in a [`Graph`] is assigned a new random id
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId {
    node_id: u64,
    function_id: FunctionId,
}

impl NodeId {
    // /// Return a new random `NodeId`
    // ///
    // /// ```
    // /// # use quakk::NodeId;
    // ///
    // /// assert_ne!(NodeId::new_random(), NodeId::new_random());
    // /// ```
    // pub(crate) fn new_random(function_id: FunctionId) -> Self {
    //     Self {
    //         node_id: RandomState::new().build_hasher().finish(),
    //         function_id,
    //     }
    // }

    pub(crate) fn zero(function_id: FunctionId) -> Self {
        Self {
            node_id: 0,
            function_id,
        }
    }

    pub(crate) fn checked_increment(&self) -> Option<Self> {
        self.node_id.checked_add(1).map(|node_id| NodeId {
            node_id,
            function_id: self.function_id,
        })
    }

    pub fn function_id(&self) -> FunctionId {
        self.function_id
    }

    pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
        PortId::Node(self.node_port_id(label))
    }

    pub fn node_port_id(&self, label: impl Into<PortLabel>) -> NodePortId {
        NodePortId::new(*self, label.into())
    }

    pub fn out(&self) -> NodePortId {
        self.node_port_id("out")
    }

    pub fn r#in(&self) -> NodePortId {
        self.node_port_id("in")
    }
}

/// Identifies a [`Function`]
///
/// Each new function declared in a [`Graph `] is assigned a new random id
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId {
    id: u64,
}

impl FunctionId {
    pub(crate) const ZERO: FunctionId = FunctionId { id: 0 };

    // /// Return a new random `FunctionId`
    // /// ```
    // /// # use quakk::FunctionId;
    // ///
    // /// assert_ne!(FunctionId::new_random(), FunctionId::new_random());
    // /// ```
    // pub fn new_random() -> Self {
    //     Self {
    //         id: RandomState::new().build_hasher().finish(),
    //     }
    // }

    pub fn as_u64(&self) -> u64 {
        self.id
    }

    #[must_use]
    pub(crate) fn checked_increment(&self) -> Option<Self> {
        self.id.checked_add(1).map(|id| FunctionId { id })
    }

    pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
        PortId::Function(self.function_port_id(label))
    }

    pub fn function_port_id(&self, label: impl Into<PortLabel>) -> FunctionPortId {
        FunctionPortId::new(*self, label.into())
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionId({})", &self.id)
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
    fn from(v: &str) -> Self {
        Self::new(v)
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

// /// Point to either a node port or function port
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum VertexPortId {
//     Node(NodePortId),
//     Function(FunctionPortId),
// }

// impl VertexPortId {
//     pub fn port_label(&self) -> PortLabel {
//         match self {
//             Self::Function(port_id) => port_id.label.clone(),
//             Self::Node(port_id) => port_id.label.clone(),
//         }
//     }

//     pub fn node_id(&self) -> Option<NodeId> {
//         match self {
//             Self::Node(node_port_id) => Some(node_port_id.node_id),
//             _ => None,
//         }
//     }

//     pub fn function_id(&self) -> Option<FunctionId> {
//         match self {
//             Self::Function(function_port_id) => Some(function_port_id.function_id),
//             _ => None,
//         }
//     }

//     pub fn as_vertex_id(&self) -> VertexId {
//         match self {
//             Self::Node(node_port_id) => VertexId::Node(node_port_id.node_id),
//             Self::Function(function_port_id) => VertexId::Function(function_port_id.function_id),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortId {
    Node(NodePortId),
    Function(FunctionPortId),
}

impl PortId {
    pub fn function_id(&self) -> FunctionId {
        match self {
            Self::Node(node_port_id) => node_port_id.id().function_id(),
            Self::Function(function_port_id) => function_port_id.function_id(),
        }
    }

    pub fn is_node(&self) -> bool {
        matches!(*self, Self::Node(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(*self, Self::Function(_))
    }
}

impl From<FunctionPortId> for PortId {
    fn from(v: FunctionPortId) -> Self {
        Self::Function(v)
    }
}

impl From<NodePortId> for PortId {
    fn from(v: NodePortId) -> Self {
        Self::Node(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodePortId {
    id: NodeId,
    label: PortLabel,
}

impl NodePortId {
    pub fn new(node_id: NodeId, label: PortLabel) -> Self {
        Self { id: node_id, label }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn label(&self) -> &PortLabel {
        &self.label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionPortId {
    id: FunctionId,
    label: PortLabel,
}

impl FunctionPortId {
    pub fn new(function_id: FunctionId, label: PortLabel) -> Self {
        Self {
            id: function_id,
            label,
        }
    }

    pub fn function_id(&self) -> FunctionId {
        self.id
    }

    pub fn label(&self) -> &PortLabel {
        &self.label
    }
}
