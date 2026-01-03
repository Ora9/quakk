use quakk::{Node, Quakk, numeric::*};

fn main() {
    let qk = Quakk::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        let number_a = graph.insert(Box::new(Number::new()));
        let number_b = graph.insert(Box::new(Number::new()));
        let number_c = graph.insert(Box::new(Number::new()));

        let mult = graph.insert(Box::new(Multiply::new()));
        let add = graph.insert(Box::new(Add::new()));

        let _ = graph.patch(
            number_a.out_id_for("out").unwrap(),
            mult.in_id_for("term1").unwrap(),
        );
        let _ = graph.patch(
            number_b.out_id_for("out").unwrap(),
            mult.in_id_for("term2").unwrap(),
        );
        let _ = graph.patch(
            mult.out_id_for("out").unwrap(),
            add.in_id_for("term1").unwrap(),
        );
        let _ = graph.patch(
            number_c.out_id_for("out").unwrap(),
            add.in_id_for("term2").unwrap(),
        );

        let num_out = graph.graph_out_in_id_for("numeric").unwrap();
        let _ = graph.patch(add.out_id_for("out").unwrap(), num_out);

        dbg!(graph);
    }

    dbg!(qk.fold_for("number_out").unwrap());
}
