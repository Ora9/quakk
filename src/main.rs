use quack::{Graph, Node, node::{Multiply, Number}, };

fn main() {
    let mut graph = Graph::new();

    let number = graph.insert(Box::new(Number::new()));
    let mult = graph.insert(Box::new(Multiply::new()));

    dbg!(&graph);
    dbg!(mult);
    dbg!(number);

    // graph.evaluate();
}
