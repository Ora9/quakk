use crate::{FunctionId, FunctionPortId, PortId, PortLabel};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub color: u32,
}

#[derive(Debug, Clone)]
pub struct FunctionHandle {
    id: FunctionId,
    function: Function,
}

impl FunctionHandle {
    pub fn new(function_id: FunctionId, function: Function) -> FunctionHandle {
        FunctionHandle {
            id: function_id,
            function,
        }
    }

    pub fn id(&self) -> FunctionId {
        self.id
    }

    pub fn function(&self) -> Function {
        self.function.clone()
    }

    pub fn port_id(&self, label: impl Into<PortLabel>) -> PortId {
        PortId::Function(FunctionPortId::new(self.id, label.into()))
    }

    // fn fold_for(&self, graph: Graph, label: impl Into<PortLabel>) -> Result<DataBox, anyhow::Error> {

    //     self.function.
    // }
}
