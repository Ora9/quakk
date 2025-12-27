use quack_sth::{
    Node, Quack,
    node::{Multiply, Number},
};

fn main() {
    let qk = Quack::new();

    {
        let mut graph = qk.graph.lock().unwrap();

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

        let numout = graph.graph_out_id_for("number_out").unwrap();

        let _ = graph.patch(mult.id_for("out").unwrap(), numout);
    }

    // dbg!(&mult);
    // dbg!(&number_a);
    // dbg!(&qk.graph);
    // qk.graph.evaluate();

    qk.evaluate_for("number_out").unwrap();
}
