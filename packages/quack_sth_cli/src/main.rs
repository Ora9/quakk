use quack_sth::{Node, Quack};

use numeric::*;

fn main() {
    let qk = Quack::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        let number_a = graph.insert(Box::new(Number::new()));
        let number_b = graph.insert(Box::new(Number::new()));
        let number_c = graph.insert(Box::new(Number::new()));

        let mult = graph.insert(Box::new(Multiply::new()));
        let add = graph.insert(Box::new(Add::new()));

        let _ = graph.patch(
            number_a.id_for("out").unwrap(),
            mult.id_for("term1").unwrap(),
        );
        let _ = graph.patch(
            number_b.id_for("out").unwrap(),
            mult.id_for("term2").unwrap(),
        );
        let _ = graph.patch(mult.id_for("out").unwrap(), add.id_for("term1").unwrap());
        let _ = graph.patch(
            number_c.id_for("out").unwrap(),
            add.id_for("term2").unwrap(),
        );

        let num_out = graph.graph_out_id_for("number_out").unwrap();

        let _ = graph.patch(add.id_for("out").unwrap(), num_out);
    }

    dbg!(qk.evaluate_for("number_out").unwrap());
}
