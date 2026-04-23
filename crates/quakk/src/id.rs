use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum FoldableId {
    Node(NodeId),
    Function(FunctionId),
}

impl FoldableId {
    pub fn port_id(&self, port_label: impl Into<PortLabel>) -> PortId {
        match self {
            FoldableId::Node(node_id) => node_id.port(port_label),
            FoldableId::Function(function_id) => function_id.port(port_label),
        }
    }
}

impl From<PortId> for FoldableId {
    fn from(v: PortId) -> Self {
        match v {
            PortId::Node(node_port_id) => Self::Node(node_port_id.id()),
            PortId::Function(function_port_id) => Self::Function(function_port_id.id()),
        }
    }
}

impl From<NodeId> for FoldableId {
    fn from(v: NodeId) -> Self {
        Self::Node(v)
    }
}

impl From<FunctionId> for FoldableId {
    fn from(v: FunctionId) -> Self {
        Self::Function(v)
    }
}

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

    pub fn port(&self, port_label: impl Into<PortLabel>) -> PortId {
        PortId::Node(self.node_port_id(port_label))
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

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn{}-nd{}", self.function_id.id, self.node_id)
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

    pub fn port(&self, port_label: impl Into<PortLabel>) -> PortId {
        PortId::Function(self.as_function_port_id(port_label))
    }

    pub fn as_function_port_id(&self, port_label: impl Into<PortLabel>) -> FunctionPortId {
        FunctionPortId::new(*self, port_label.into())
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FunctionId({})", &self.id)
    }
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn{}", self.id)
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

impl Display for PortLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label.as_str())
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

    fn from_label(port_label: impl Into<PortLabel>) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_str(port_label.into().as_str())
    }

    fn to_str(&self) -> &str;

    fn to_label(&self) -> PortLabel {
        PortLabel::new(Self::to_str(self))
    }

    fn direction(&self) -> PortDirection;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortId {
    Node(NodePortId),
    Function(FunctionPortId),
}

impl PortId {
    pub fn function_id(&self) -> FunctionId {
        match self {
            Self::Node(node_port_id) => node_port_id.id().function_id(),
            Self::Function(function_port_id) => function_port_id.id(),
        }
    }

    pub fn port_label(&self) -> &PortLabel {
        match self {
            Self::Node(node_port_id) => node_port_id.label(),
            Self::Function(function_port_id) => function_port_id.label(),
        }
    }

    pub fn is_node(&self) -> bool {
        matches!(*self, Self::Node(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(*self, Self::Function(_))
    }
}

/// fn5-nd87-in
/// fn2-number_in
impl Display for PortId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function(function_port_id) => {
                write!(f, "{}-{}", function_port_id.id, function_port_id.label)
            }
            Self::Node(node_port_id) => {
                write!(
                    f,
                    "{}-{}-{}",
                    node_port_id.id.function_id, node_port_id.id.node_id, node_port_id.label
                )
            }
        }
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

    pub fn id(&self) -> FunctionId {
        self.id
    }

    pub fn label(&self) -> &PortLabel {
        &self.label
    }
}
