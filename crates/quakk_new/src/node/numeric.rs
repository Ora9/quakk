use anyhow::{Context, anyhow};

use crate::{DataBox, Node, NodeBox, Port, PortDirection, PortLabel};

pub enum NumericPorts {
    Value,
    Out,
}

impl Port for NumericPorts {
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
    value: f32,
}

impl Node for NumericConstant {
    fn init() -> NodeBox {
        NodeBox::new(Box::new(NumericConstant { value: 2.0 }))
    }

    fn title(&self) -> &str {
        "Numeric Constant"
    }

    fn mutate(&mut self, port_label: PortLabel, value: DataBox) -> Result<(), anyhow::Error> {
        let port = NumericPorts::from_label(port_label).context("not a valid port")?;

        match port {
            NumericPorts::Value => {
                self.value = value.into_f32().context("while trying to mutate")?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticsOperation {
    Addition = 0,
    Substraction = 1,
    Multiplication = 2,
    Division = 3,
}

impl TryFrom<f32> for ArithmeticsOperation {
    type Error = anyhow::Error;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
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

impl TryFrom<DataBox> for ArithmeticsOperation {
    type Error = anyhow::Error;
    fn try_from(value: DataBox) -> Result<Self, Self::Error> {
        Self::try_from(value.into_f32()?)
    }
}

impl Into<DataBox> for ArithmeticsOperation {
    fn into(self) -> DataBox {
        DataBox::new(self as u32 as f32)
    }
}

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
    term1: f32,
    term2: f32,
}

impl Node for Arithmetics {
    fn init() -> NodeBox
    where
        Self: Sized,
    {
        NodeBox::new(Box::new(Arithmetics {
            operation: ArithmeticsOperation::Addition,
            term1: 0.0,
            term2: 0.0,
        }))
    }

    fn title(&self) -> &str {
        "Arithmetics"
    }

    fn mutate(&mut self, port_label: PortLabel, value: DataBox) -> Result<(), anyhow::Error> {
        let port = ArithmeticsPorts::from_label(port_label).context("not a valid port")?;

        match port {
            ArithmeticsPorts::Term1 => {
                self.term1 = value.into_f32().context("term1 is invalid")?;
                Ok(())
            }
            ArithmeticsPorts::Term2 => {
                self.term2 = value.into_f32().context("term2 is invalid")?;
                Ok(())
            }
            ArithmeticsPorts::Operation => {
                dbg!(&value);
                self.operation = ArithmeticsOperation::try_from(
                    value.into_f32().context("operation is invalid")?,
                )?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }
}
