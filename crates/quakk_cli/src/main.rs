use quakk::{
    Node, Quakk,
    numeric::{Arithmetics, ArithmeticsOperation, NumericConstant},
};

fn main() -> Result<(), anyhow::Error> {
    let mut qk = Quakk::new();

    qk.graph_mut(|graph| {
        let number_a = graph.insert_node(NumericConstant::init().mutate("in", 2.0)?);
        let number_b = graph.insert_node(NumericConstant::init().mutate("in", 3.0)?);
        let number_c = graph.insert_node(NumericConstant::init().mutate("in", 5.0)?);

        let mult = graph.insert_node(
            Arithmetics::init().mutate("operation", ArithmeticsOperation::Multiplication)?,
        );
        let add = graph
            .insert_node(Arithmetics::init().mutate("operation", ArithmeticsOperation::Addition)?);

        let _ = graph.patch(number_a.out(), mult.port_id("term1"));
        let _ = graph.patch(number_b.out(), mult.port_id("term2"));
        let _ = graph.patch(mult.out(), add.port_id("term1"));
        let _ = graph.patch(number_c.out(), add.port_id("term2"));

        let number_out = graph.main_function_handle().port_id("number_out");

        let _ = graph.patch(add.port_id("out"), number_out);

        // dbg!(&graph);

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
        Ok::<(), anyhow::Error>(())
    })?;

    let _ = qk.fold_for("number_out");
    // dbg!(qk.fold_for("number_out").unwrap());

    Ok(())
}
