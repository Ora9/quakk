use std::ops::{Add, Div, Mul, Sub};

use anyhow::{Context, anyhow};

use crate::{Data, LasyFold, Node, NodeTrait, Number, Port, PortDirection, PortLabel};

pub enum NumericConstantPorts {
    Value,
    Out,
}

impl Port for NumericConstantPorts {
    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match str {
            "value" => Some(Self::Value),
            "out" => Some(Self::Out),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Value => "value",
            Self::Out => "out",
        }
    }

    fn direction(&self) -> PortDirection {
        match self {
            Self::Value => PortDirection::In,
            Self::Out => PortDirection::Out,
        }
    }
}

#[derive(Debug)]
pub struct NumericConstant {
    value: Number,
}

impl NodeTrait for NumericConstant {
    fn init() -> Node {
        Node::new(Box::new(NumericConstant { value: 2.0 }))
    }

    fn title(&self) -> &str {
        "Numeric Constant"
    }

    fn mutate(&mut self, port: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = NumericConstantPorts::from_label(port).context("not a valid port")?;

        match port {
            NumericConstantPorts::Value => {
                self.value = value.into_number().context("while trying to mutate")?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }

    fn fold(&mut self, _port: PortLabel, _lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        Ok(self.value.into())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArithmeticsOperation {
    Addition = 0,
    Substraction = 1,
    Multiplication = 2,
    Division = 3,
}

impl TryFrom<Number> for ArithmeticsOperation {
    type Error = anyhow::Error;

    fn try_from(value: Number) -> Result<Self, Self::Error> {
        use ArithmeticsOperation::*;

        match value as u32 {
            x if x == Addition as u32 => Ok(Addition),
            x if x == Substraction as u32 => Ok(Substraction),
            x if x == Multiplication as u32 => Ok(Multiplication),
            x if x == Division as u32 => Ok(Division),
            _ => Err(anyhow!("not a valid arithmetic operation")),
        }
    }
}

impl TryFrom<Data> for ArithmeticsOperation {
    type Error = anyhow::Error;
    fn try_from(value: Data) -> Result<Self, Self::Error> {
        Self::try_from(value.into_number()?)
    }
}

impl From<ArithmeticsOperation> for Number {
    fn from(value: ArithmeticsOperation) -> Self {
        value as usize as Number
    }
}

impl From<ArithmeticsOperation> for Data {
    fn from(value: ArithmeticsOperation) -> Self {
        Data::new(value as usize as Number)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArithmeticsPorts {
    Operation,
    Term1,
    Term2,
    Out,
}

impl Port for ArithmeticsPorts {
    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match str {
            "operation" => Some(Self::Operation),
            "Term1" => Some(Self::Term1),
            "Term2" => Some(Self::Term2),
            "out" => Some(Self::Out),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::Operation => "operation",
            Self::Term1 => "term1",
            Self::Term2 => "term2",
            Self::Out => "out",
        }
    }

    fn direction(&self) -> PortDirection {
        match self {
            Self::Operation | Self::Term1 | Self::Term2 => PortDirection::In,
            Self::Out => PortDirection::Out,
        }
    }
}

#[derive(Debug)]
pub struct Arithmetics {
    operation: ArithmeticsOperation,
    term1: Number,
    term2: Number,
}

impl NodeTrait for Arithmetics {
    fn init() -> Node
    where
        Self: Sized,
    {
        Node::new(Box::new(Arithmetics {
            operation: ArithmeticsOperation::Addition,
            term1: 0.0,
            term2: 0.0,
        }))
    }

    fn title(&self) -> &str {
        "Arithmetics"
    }

    fn mutate(&mut self, port: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = ArithmeticsPorts::from_label(port).context("not a valid port")?;

        match port {
            ArithmeticsPorts::Term1 => {
                self.term1 = value.into_number().context("term1 is invalid")?;
                Ok(())
            }
            ArithmeticsPorts::Term2 => {
                self.term2 = value.into_number().context("term2 is invalid")?;
                Ok(())
            }
            ArithmeticsPorts::Operation => {
                self.operation = ArithmeticsOperation::try_from(
                    value.into_number().context("operation is invalid")?,
                )?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }

    fn fold(&mut self, port: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        match ArithmeticsPorts::from_label(port) {
            Some(ArithmeticsPorts::Out) => {
                let term1 = lasy_fold.get_in_as_number("term1").unwrap_or(self.term1);
                let term2 = lasy_fold.get_in_as_number("term2").unwrap_or(self.term2);
                let operation = lasy_fold
                    .get_in_as_number("operation")
                    .unwrap_or(self.operation.into())
                    .try_into()
                    .context("`operation` is not a valid arithmetic operation")?;

                use ArithmeticsOperation::*;
                let res = match operation {
                    Addition => term1.add(term2),
                    Substraction => term1.sub(term2),
                    Multiplication => term1.mul(term2),
                    Division => term1.div(term2),
                };
                Ok(res.into())
            }
            _ => Err(anyhow!("not a valid output port")),
        }
    }
}
