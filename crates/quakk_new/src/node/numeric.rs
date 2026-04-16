use anyhow::{Context, anyhow, bail};

use crate::{Data, LasyFold, Node, NodeBox, Number, Port, PortDirection, PortId, PortLabel};

pub enum NumericConstantPorts {
    In,
    Out,
}

impl Port for NumericConstantPorts {
    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match str {
            "in" => Some(Self::In),
            "out" => Some(Self::Out),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::In => "in",
            Self::Out => "out",
        }
    }

    fn direction(&self) -> PortDirection {
        match self {
            Self::In => PortDirection::In,
            Self::Out => PortDirection::Out,
        }
    }
}

#[derive(Debug)]
pub struct NumericConstant {
    value: Number,
}

impl Node for NumericConstant {
    fn init() -> NodeBox {
        NodeBox::new(Box::new(NumericConstant { value: 2.0 }))
    }

    fn title(&self) -> &str {
        "Numeric Constant"
    }

    fn mutate(&mut self, port_label: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = NumericConstantPorts::from_label(port_label).context("not a valid port")?;

        dbg!(&value);
        match port {
            NumericConstantPorts::In => {
                self.value = value.into_number().context("while trying to mutate")?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }

    fn fold(&mut self, port_out: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        Err(anyhow!("ho that's unimplemented"))
    }
}

#[derive(Debug)]
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

impl Into<Data> for ArithmeticsOperation {
    fn into(self) -> Data {
        Data::new(self as u32 as Number)
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

    fn mutate(&mut self, port_label: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = ArithmeticsPorts::from_label(port_label).context("not a valid port")?;

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

    fn fold(&mut self, port_out: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        if let Some(port) = ArithmeticsPorts::from_label(port_out)
            && port != ArithmeticsPorts::Out
        {
            return Err(anyhow!("no a valid output port"));
        }

        self.mutate("term1".into(), lasy_fold.get_in("term1")?);

        dbg!(self);

        Ok(Data::new(2.0))
    }
}
