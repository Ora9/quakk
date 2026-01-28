use quakk::{LasyFold, Node, Quakk, id::InId, numeric::*};

fn main() {
    let qk = Quakk::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        let number_a = graph.insert(Box::new(Number::new()));
        let number_b = graph.insert(Box::new(Number::new()));
        let number_c = graph.insert(Box::new(Number::new()));

        let mult = graph.insert(Box::new(Multiply::new()));
        let add = graph.insert(Box::new(Add::new()));

        dbg!(mult.node_in_id(&MultiplyInId::Term1));
        dbg!(number_a.node_out_id(&NumberOutId::Prout(55)));

        // dbg!(prout.unwrap().in_id());

        // let _ = graph.patch(
        //     mult.in_id(MultiplyInId::Term1)?,
        // );
        // let _ = graph.patch(
        //     number_a.out_id_for("out").unwrap(),
        //     mult.in_id_for("term1").unwrap(),
        // );
        // let _ = graph.patch(
        //     number_b.out_id_for("out").unwrap(),
        //     mult.in_id_for("term2").unwrap(),
        // );
        // let _ = graph.patch(
        //     mult.out_id_for("out").unwrap(),
        //     add.in_id_for("term1").unwrap(),
        // );
        // let _ = graph.patch(
        //     number_c.out_id_for("out").unwrap(),
        //     add.in_id_for("term2").unwrap(),
        // );

        // let num_out = graph.graph_out_in_id_for("numeric").unwrap();
        // let _ = graph.patch(add.out_id_for("out").unwrap(), num_out);

        // dbg!(graph);
    }

    // dbg!(qk.fold_for("number_out").unwrap());
}
