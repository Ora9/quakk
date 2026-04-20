use anyhow::{Context, anyhow};

use crate::{Data, Node, NodeTrait, Port, PortDirection, PortLabel, Text};

enum TextConstantPort {
    Value,
    Out,
}

impl Port for TextConstantPort {
    fn direction(&self) -> PortDirection {
        match self {
            TextConstantPort::Value => PortDirection::In,
            TextConstantPort::Out => PortDirection::Out,
        }
    }

    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match str {
            "value" => Some(TextConstantPort::Value),
            "out" => Some(TextConstantPort::Out),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            TextConstantPort::Value => "value",
            TextConstantPort::Out => "out",
        }
    }
}

#[derive(Debug)]
pub struct TextConstant {
    value: Text,
}

impl Default for TextConstant {
    fn default() -> Self {
        Self { value: "".into() }
    }
}

impl NodeTrait for TextConstant {
    fn title(&self) -> &str {
        "Text Constant"
    }

    fn init() -> Node
    where
        Self: Sized,
    {
        Node::new(Box::new(TextConstant::default()))
    }

    fn mutate(&mut self, port: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = TextConstantPort::from_label(port).context("not a valid port")?;

        match port {
            TextConstantPort::Value => {
                self.value = value.into_text().context("could not mutate")?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }

    fn fold(&mut self, port: PortLabel, lasy_fold: crate::LasyFold) -> Result<Data, anyhow::Error> {
        Ok(self.value.clone().into())
    }
}
