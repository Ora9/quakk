#![allow(dead_code)]

use quack::{Graph, Node, node::{StringValue, TrimString}};

fn main() {
    let mut graph = Graph::new();

    let strvalue = graph.insert(Box::new(StringValue::new()));
    let strim = graph.insert(Box::new(TrimString::new()));

    dbg!(&graph);
    dbg!(strvalue);

    // graph.evaluate();
}
