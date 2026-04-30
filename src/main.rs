use chumsky::Parser;
use dtk_pal::util::parsing;
use dtk_pal::{splits, symbols};
use eyre::Result;

fn main() -> Result<()> {
    main_splits()
}

fn main_symbols() -> Result<()> {
    let input = include_str!("../../hidden_mansion_src/config/GLME01/symbols.txt");

    let obj_symbols = match symbols::parser().parse(input).into_result() {
        Ok(obj_symbols) => obj_symbols,
        Err(errs) => return parsing::print_errors(&errs, input),
    };

    let json = serde_json::to_string_pretty(&obj_symbols)?;

    println!("{json}");

    Ok(())
}

fn main_splits() -> Result<()> {
    let input = include_str!("../../hidden_mansion_src/config/GLME01/splits.txt");

    let splits = match splits::parser().parse(input).into_result() {
        Ok(splits) => splits,
        Err(errs) => return parsing::print_errors(&errs, input),
    };

    println!("{splits:#?}");

    Ok(())
}
