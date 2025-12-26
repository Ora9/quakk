use quack_sth::{
    Graph, Node,
    node::{Multiply, Number},
};

fn main() {
    let mut graph = Graph::new();

    let number_a = graph.insert(Box::new(Number::new()));
    let number_b = graph.insert(Box::new(Number::new()));
    let mult = graph.insert(Box::new(Multiply::new()));

    let _ = graph.patch(
        number_a.id_for("out").unwrap(),
        mult.id_for("term1").unwrap(),
    );
    let _ = graph.patch(
        number_b.id_for("out").unwrap(),
        mult.id_for("term2").unwrap(),
    );

    let _ = graph.patch(
        mult.id_for("out").unwrap(),
        graph.out_id_for("number_out").unwrap(),
    );

    // dbg!(&mult);
    // dbg!(&number_a);
    dbg!(&graph);
    // graph.evaluate();
}
