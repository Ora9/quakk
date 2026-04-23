use quakk::{
    Function, FunctionDef, NodeTrait, Number, Quakk, Text,
    numeric::{Arithmetics, ArithmeticsOperation, NumericConstant},
    text::{TextConstant, TextSplit},
};

fn main() -> Result<(), anyhow::Error> {
    let mut qk = Quakk::new();

    qk.graph_mut(|graph| {
        let patate = graph.insert_function(FunctionDef {
            name: "patate".to_string(),
            color: 88,
        });

        let main_function = graph.main_function_id();

        let num_a =
            graph.insert_in_main(NumericConstant::init().mutate("value", Number::from(2.0))?);
        let num_b =
            graph.insert_in_main(NumericConstant::init().mutate("value", Number::from(3.0))?);
        let num_c =
            graph.insert_in_main(NumericConstant::init().mutate("value", Number::from(1.0))?);

        let mult = graph.insert_in_main(
            Arithmetics::init().mutate("operation", ArithmeticsOperation::Multiplication)?,
        );
        let add = graph.insert_in_main(
            Arithmetics::init().mutate("operation", ArithmeticsOperation::Addition)?,
        );

        let text_const =
            graph.insert_in_main(TextConstant::init().mutate("text", Text::from("Hello World"))?);

        let text_split = graph.insert_in_main(TextSplit::init().mutate("at", Number::from(0.0))?);

        graph.patch(num_a.out(), mult.port("term1"))?;
        graph.patch(num_b.out(), mult.port("term2"))?;

        graph.patch(mult.out(), add.port("term1"))?;
        graph.patch(num_c.out(), add.port("term2"))?;

        graph.patch(add.out(), text_split.port("at"))?;
        graph.patch(text_const.out(), text_split.port("text"))?;
        graph.patch(text_split.port("start"), main_function.port("text_out"))?;

        // let patate_num_a = graph.insert_in(
        //     patate,
        //     NumericConstant::init().mutate("value", Number::from(8.55))?,
        // );
        // let patate_num_b = graph.insert_in(
        //     patate,
        //     NumericConstant::init().mutate("value", Number::from(1312.161))?,
        // );

        // let patate_add = graph.insert_in(
        //     patate,
        //     Arithmetics::init().mutate("operation", ArithmeticsOperation::Addition)?,
        // );

        // let _ = graph.patch(patate.port("number_in"), patate_add.port("term2"));
        // let _ = graph.patch(patate_num_a.out(), patate_add.port("term1"));
        // let _ = graph.patch(patate_add.out(), main_function.port("number_out"));

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

        // dbg!(&graph);
        Ok::<(), anyhow::Error>(())
    })?;

    // let _ = qk.fold_for("number_out");
    dbg!(qk.fold_for("text_out"));

    Ok(())
}
