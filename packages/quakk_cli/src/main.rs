use quakk::{
    GraphOut, GraphOutInId, GraphOutOutId, LasyFold, Node, Quakk,
    id::InId,
    numeric::*,
    textual::{TextConstant, TextConstantOutId, TextSplit, TextSplitInId, TextSplitOutId},
};

fn main() {
    let qk = Quakk::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        let number_a = graph.insert(Box::new(NumericConstant::new(2.0)));
        let number_b = graph.insert(Box::new(NumericConstant::new(3.0)));
        let number_c = graph.insert(Box::new(NumericConstant::new(2.0)));

        let mult = graph.insert(Box::new(Arithmetics::new(
            ArithmeticOperation::Multiplication,
        )));
        let add = graph.insert(Box::new(Arithmetics::new(
            ArithmeticOperation::Substraction,
        )));

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

        let textconst = graph.insert(Box::new(TextConstant::new("Hello World!".to_string())));
        let textsplit = graph.insert(Box::new(TextSplit::default()));

        let _ = graph.patch(
            add.node_out_id(&ArithmeticsOutId::Out).unwrap(),
            textsplit.node_in_id(&TextSplitInId::At).unwrap(),
        );

        let _ = graph.patch(
            textconst.node_out_id(&TextConstantOutId::Out).unwrap(),
            textsplit.node_in_id(&TextSplitInId::Text).unwrap(),
        );

        let num_out = graph.graph_out_in_id(&GraphOutInId::Numeric).unwrap();
        let _ = graph.patch(
            textsplit.node_out_id(&TextSplitOutId::Start).unwrap(),
            num_out,
        );

        dbg!(graph);
    }

    dbg!(qk.fold_for(GraphOutOutId::Numeric).unwrap());
}
