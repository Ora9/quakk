use quakk::{GraphOut, GraphOutInId, GraphOutOutId, LasyFold, Node, Quakk, id::InId, numeric::*};

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
            number_a.node_out_id(&NumberOutId::Out).unwrap(),
            mult.node_in_id(&MultiplyInId::Term1).unwrap(),
        );
        let _ = graph.patch(
            number_b.node_out_id(&NumberOutId::Out).unwrap(),
            mult.node_in_id(&MultiplyInId::Term2).unwrap(),
        );
        let _ = graph.patch(
            mult.node_out_id(&MultiplyOutId::Out).unwrap(),
            add.node_in_id(&AddInId::Term1).unwrap(),
        );
        let _ = graph.patch(
            number_c.node_out_id(&NumberOutId::Out).unwrap(),
            add.node_in_id(&AddInId::Term2).unwrap(),
        );

        let num_out = graph.graph_out_in_id(&GraphOutInId::Numeric).unwrap();
        let _ = graph.patch(add.node_out_id(&AddOutId::Out).unwrap(), num_out);

        dbg!(graph);
    }

    dbg!(qk.fold_for(GraphOutOutId::Numeric).unwrap());
}
