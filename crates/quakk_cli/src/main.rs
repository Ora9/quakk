use quakk::{
    Node, NodeBox, Quakk,
    numeric::{Arithmetics, ArithmeticsOperation, NumericConstant},
};

fn main() -> Result<(), anyhow::Error> {
    let qk = Quakk::new();

    {
        let mut graph = qk.graph.lock().unwrap();

        dbg!(&graph);

        let number_a = graph.insert_node(NumericConstant::init().mutate("value", 2.0)?);
        let number_b = graph.insert_node(NumericConstant::init().mutate("value", 2.0)?);
        let number_c = graph.insert_node(NumericConstant::init().mutate("value", 5.0)?);

        let mult = graph.insert_node(
            Arithmetics::init().mutate("operation", ArithmeticsOperation::Multiplication)?,
        );
        let mult = graph
            .insert_node(Arithmetics::init().mutate("operation", ArithmeticsOperation::Addition)?);

        dbg!(&graph);

        // let _ = graph.patch(
        //     number_a.(&NumericConstantOut::Out),
        //     mult.node_in_id(&ArithmeticsIn::Term1),
        // );
        //     let _ = graph.patch(
        //         number_b.node_out_id(&NumericConstantOut::Out),
        //         mult.node_in_id(&ArithmeticsIn::Term2),
        //     );
        //     let _ = graph.patch(
        //         mult.node_out_id(&ArithmeticsOut::Out),
        //         add.node_in_id(&ArithmeticsIn::Term1),
        //     );
        //     let _ = graph.patch(
        //         number_c.node_out_id(&NumericConstantOut::Out),
        //         add.node_in_id(&ArithmeticsIn::Term2),
        //     );

        //     let textconst = graph.insert(Box::new(TextConstant::new("Hello World!".to_string())));
        //     let textsplit = graph.insert(Box::new(TextSplit::default()));

        //     let _ = graph.patch(
        //         add.node_out_id(&ArithmeticsOut::Out),
        //         textsplit.node_in_id(&TextSplitIn::At),
        //     );

        //     let _ = graph.patch(
        //         textconst.node_out_id(&TextConstantOut::Out),
        //         textsplit.node_in_id(&TextSplitIn::Text),
        //     );

        //     let num_out = graph.graph_out_in_id(&GraphOutIn::Numeric);
        //     let _ = graph.patch(textsplit.node_out_id(&TextSplitOut::Start), num_out);

        //     dbg!(graph);
    }

    // dbg!(qk.fold_for(GraphOutOut::Numeric).unwrap());

    Ok(())
}
