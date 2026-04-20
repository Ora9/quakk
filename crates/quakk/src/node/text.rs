use anyhow::{Context, anyhow};

use crate::{Data, LasyFold, Node, NodeTrait, Number, Port, PortDirection, PortLabel, Text};

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

#[derive(Debug, Default)]
pub struct TextConstant {
    value: Text,
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

    fn fold(&mut self, port: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        Ok(self.value.clone().into())
    }
}

enum TextSplitPort {
    At,
    Text,
    Start,
    End,
}

impl Port for TextSplitPort {
    fn direction(&self) -> PortDirection {
        match self {
            Self::At | Self::Text => PortDirection::In,
            Self::Start | Self::End => PortDirection::Out,
        }
    }

    fn from_str(str: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match str {
            "at" => Some(Self::At),
            "text" => Some(Self::Text),
            "start" => Some(Self::Start),
            "end" => Some(Self::End),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Self::At => "at",
            Self::Text => "text",
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Default)]
pub struct TextSplit {
    at: Number,
    text: Text,
}

impl NodeTrait for TextSplit {
    fn title(&self) -> &str {
        "Text Split"
    }

    fn init() -> Node
    where
        Self: Sized,
    {
        Node::new(Box::new(Self::default()))
    }

    fn mutate(&mut self, port: PortLabel, value: Data) -> Result<(), anyhow::Error> {
        let port = TextSplitPort::from_label(port).context("not a valid port")?;

        match port {
            TextSplitPort::At => {
                self.at = value.into_number().context("could not mutate")?;
                Ok(())
            }
            TextSplitPort::Text => {
                self.text = value.into_text().context("could not mutate")?;
                Ok(())
            }
            _ => Err(anyhow!("not a valid input port")),
        }
    }

    fn fold(&mut self, port: PortLabel, lasy_fold: LasyFold) -> Result<Data, anyhow::Error> {
        let port = TextSplitPort::from_label(port).context("not a valid port")?;

        match port {
            TextSplitPort::Start | TextSplitPort::End => {
                let at = lasy_fold.get_in_as_number("at").unwrap_or(self.at);
                let text = lasy_fold
                    .get_in_as_text("text")
                    .unwrap_or(self.text.clone());

                let res = match port {
                    TextSplitPort::Start => text.split_at(at.into()).0,
                    TextSplitPort::End => text.split_at(at.into()).1,
                    _ => unreachable!(),
                };

                Ok(Text::from(res).into())
            }
            _ => Err(anyhow!("not a valid output port")),
        }
    }
}
