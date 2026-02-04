use quakk::{GraphOut, GraphOutInId, GraphOutOutId, LasyFold, Node, Quakk, id::InId, numeric::*};

fn main() {
    let qk = Quakk::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        let number_a = graph.insert(Box::new(NumericConstant::new(4.0)));
        let number_b = graph.insert(Box::new(NumericConstant::new(4.0)));
        let number_c = graph.insert(Box::new(NumericConstant::new(8.0)));

        let mult = graph.insert(Box::new(Arithmetics::new(
            ArithmeticOperation::Multiplication,
        )));
        let add = graph.insert(Box::new(Arithmetics::new(ArithmeticOperation::Addition)));

        let _ = graph.patch(
            number_a.node_out_id(&NumericConstantOutId::Out).unwrap(),
            mult.node_in_id(&ArithmeticsInId::Term1).unwrap(),
        );
        let _ = graph.patch(
            number_b.node_out_id(&NumericConstantOutId::Out).unwrap(),
            mult.node_in_id(&ArithmeticsInId::Term2).unwrap(),
        );
        let _ = graph.patch(
            mult.node_out_id(&ArithmeticsOutId::Out).unwrap(),
            add.node_in_id(&ArithmeticsInId::Term1).unwrap(),
        );
        let _ = graph.patch(
            number_c.node_out_id(&NumericConstantOutId::Out).unwrap(),
            add.node_in_id(&ArithmeticsInId::Term2).unwrap(),
        );

        let num_out = graph.graph_out_in_id(&GraphOutInId::Numeric).unwrap();
        let _ = graph.patch(add.node_out_id(&ArithmeticsOutId::Out).unwrap(), num_out);

        dbg!(graph);
    }

    dbg!(qk.fold_for(GraphOutOutId::Numeric).unwrap());
}
