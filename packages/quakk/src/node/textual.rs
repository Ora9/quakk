use anyhow::{Context, anyhow};

use crate::{
    Data, LasyFold, Meta, Node,
    id::{InId, NodeId, NodeInId, NodeOutId, OutId},
};

#[derive(Debug, Default)]
pub struct TextConstant {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextConstantOutId {
    Out,
}

impl OutId for TextConstantOutId {}

impl TextConstant {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Node for TextConstant {
    fn initialize() -> Self {
        Self::default()
    }

    fn title(&self) -> &str {
        "Text Constant"
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        None
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        out_id
            .as_any()
            .downcast_ref::<TextConstantOutId>()
            .map(|out_id| NodeOutId::new(node_id, out_id))
    }

    fn fold(&self, _out_id: &dyn OutId, _lasy_fold: LasyFold, _meta: Meta) -> anyhow::Result<Data> {
        Ok(Data::new(self.value.clone()))
    }
}

#[derive(Debug, Default)]
pub struct TextSplit;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextSplitInId {
    Text,
    At,
}

impl InId for TextSplitInId {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextSplitOutId {
    Start,
    End,
}

impl OutId for TextSplitOutId {}

impl Node for TextSplit {
    fn title(&self) -> &str {
        "Text Split"
    }

    fn initialize() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn node_in_id(&self, in_id: &dyn InId, node_id: NodeId) -> Option<NodeInId> {
        in_id
            .as_any()
            .downcast_ref::<TextSplitInId>()
            .map(|out_id| NodeInId::new(node_id, in_id))
    }

    fn node_out_id(&self, out_id: &dyn OutId, node_id: NodeId) -> Option<NodeOutId> {
        out_id
            .as_any()
            .downcast_ref::<TextSplitOutId>()
            .map(|out_id| NodeOutId::new(node_id, out_id))
    }

    fn fold(&self, out_id: &dyn OutId, lasy_fold: LasyFold, meta: Meta) -> anyhow::Result<Data> {
        let text = lasy_fold
            .get_in(&TextSplitInId::Text, meta)?
            .into_string()
            .context("type mismatch for Text, expected String")?;
        let at = lasy_fold
            .get_in(&TextSplitInId::At, meta)?
            .into_f32()
            .context("type mismatch for Term2, expected f32")?;

        dbg!(at as usize);
        match out_id.as_any().downcast_ref::<TextSplitOutId>() {
            Some(out_id) => {
                let split = text.split_at(at as usize);

                match out_id {
                    TextSplitOutId::Start => Ok(Data::new(split.0.to_string())),
                    TextSplitOutId::End => Ok(Data::new(split.1.to_string())),
                }
            }
            None => Err(anyhow!("Not a valid out_id")),
        }

        // .map(|out_id| NodeInId::new(node_id, in_id))

        // Ok(Data::new(text.split_at(at as usize)))
    }
}
