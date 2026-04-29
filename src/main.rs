use chumsky::Parser;
use dtk_pal::symbols;
use dtk_pal::util::parsing;
use eyre::Result;

fn main() -> Result<()> {
    let input = include_str!("../../hidden_mansion_src/config/GLME01/symbols.txt");

    let obj_symbols = match symbols::parser().parse(input).into_result() {
        Ok(obj_symbols) => obj_symbols,
        Err(errs) => return parsing::print_errors(&errs, input),
    };

    let json = serde_json::to_string_pretty(&obj_symbols)?;

    println!("{json}");

    Ok(())
}
