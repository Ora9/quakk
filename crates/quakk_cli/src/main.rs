use quakk::{
    GraphOut, GraphOutIn, GraphOutOut, LasyFold, Node, Quakk,
    id::InId,
    numeric::*,
    textual::{TextConstant, TextConstantOut, TextSplit, TextSplitIn, TextSplitOut},
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
            number_a.node_out_id(&NumericConstantOut::Out),
            mult.node_in_id(&ArithmeticsIn::Term1),
        );
        let _ = graph.patch(
            number_b.node_out_id(&NumericConstantOut::Out),
            mult.node_in_id(&ArithmeticsIn::Term2),
        );
        let _ = graph.patch(
            mult.node_out_id(&ArithmeticsOut::Out),
            add.node_in_id(&ArithmeticsIn::Term1),
        );
        let _ = graph.patch(
            number_c.node_out_id(&NumericConstantOut::Out),
            add.node_in_id(&ArithmeticsIn::Term2),
        );

        let textconst = graph.insert(Box::new(TextConstant::new("Hello World!".to_string())));
        let textsplit = graph.insert(Box::new(TextSplit::default()));

        let _ = graph.patch(
            add.node_out_id(&ArithmeticsOut::Out),
            textsplit.node_in_id(&TextSplitIn::At),
        );

        let _ = graph.patch(
            textconst.node_out_id(&TextConstantOut::Out),
            textsplit.node_in_id(&TextSplitIn::Text),
        );

        let num_out = graph.graph_out_in_id(&GraphOutIn::Numeric);
        let _ = graph.patch(textsplit.node_out_id(&TextSplitOut::Start), num_out);

        dbg!(graph);
    }

    dbg!(qk.fold_for(GraphOutOut::Numeric).unwrap());
}
