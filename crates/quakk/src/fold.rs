use std::{rc::Rc, sync::Mutex};

use anyhow::Context;

use crate::{Data, FoldableId, Graph, Number, PortId, PortLabel, Text};

pub enum FoldResult {
    Ok(Data),
    Error,
    Unpatched,
}

pub struct LasyFold {
    current_foldable: FoldableId,
    graph: Rc<Mutex<Graph>>,
}

impl LasyFold {
    pub fn new(graph: Rc<Mutex<Graph>>, current_foldable: FoldableId) -> Self {
        Self {
            graph,
            current_foldable,
        }
    }

    pub fn get_in(&self, port_label: impl Into<PortLabel>) -> Result<Data, anyhow::Error> {
        let graph = self
            .graph
            .lock()
            .expect("the graph has been poisoned, who was it!?");

        let target = self.current_foldable.port_id(port_label);
        let source = graph
            .edge_for_target_port(target.clone())
            .context(format!(
                "could not get the pointed edge ({:?})",
                target.clone()
            ))?
            .source();

        match source {
            PortId::Node(node_port_id) => {
                let node = graph
                    .node_for_id(node_port_id.id())
                    .context("no node")?
                    .clone();

                let foldable = FoldableId::Node(node_port_id.id());
                let label = node_port_id.label().clone();
                // only there for error management
                let source = source.clone();

                drop(graph);

                let lasy_fold = LasyFold::new(self.graph.clone(), foldable);
                node.lock()
                    .expect("this node has been poisoned !")
                    .fold(label, lasy_fold)
                    .context(format!("folding node with id {:?}", source))
            }
            PortId::Function(_function_port_id) => {
                unimplemented!("ah ça c'est pas prévu")
            }
        }
    }

    pub fn get_in_as_number(
        &self,
        port_label: impl Into<PortLabel>,
    ) -> Result<Number, anyhow::Error> {
        let port_label = port_label.into();
        let error_label = port_label.as_str();

        let num = self
            .get_in(port_label.clone())
            .context(format!("could not get `{}`", error_label))?
            .into_number()
            .context(format!("`{}` is not a valid number", error_label))?;

        Ok(num)
    }

    pub fn get_in_as_text(&self, port_label: impl Into<PortLabel>) -> Result<Text, anyhow::Error> {
        let port_label = port_label.into();
        let error_label = port_label.as_str();

        let text = self
            .get_in(port_label.clone())
            .context(format!("could not get `{}`", error_label))?
            .into_text()
            .context(format!("`{}` is not a valid text", error_label))?;

        Ok(text)
    }
}
