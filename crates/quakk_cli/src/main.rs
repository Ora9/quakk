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
        let number_b = graph.insert_node(NumericConstant::init().mutate("value", 3.0)?);
        let number_c = graph.insert_node(NumericConstant::init().mutate("value", 5.0)?);

        let mult = graph.insert_node(
            Arithmetics::init().mutate("operation", ArithmeticsOperation::Multiplication)?,
        );
        let add = graph
            .insert_node(Arithmetics::init().mutate("operation", ArithmeticsOperation::Addition)?);

        graph.patch(number_a.port_id("value"), mult.port_id("term1"));
        graph.patch(number_b.port_id("value"), mult.port_id("term2"));
        graph.patch(mult.port_id("out"), add.port_id("term1"));
        graph.patch(number_c.port_id("value"), add.port_id("term2"));

        let number_out = graph.main_function_handle().port_id("number_out");

        graph.patch(add.port_id("out"), number_out);

        dbg!(&graph);

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
