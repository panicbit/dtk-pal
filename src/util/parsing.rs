use std::fmt::Display;

use ariadne::{Label, Report, ReportKind, Source};
use chumsky::Parser;
use chumsky::error::Rich;
use eyre::Result;

pub type Extra<'a> = chumsky::extra::Err<chumsky::error::Rich<'a, char>>;
pub trait P<'a, O>: Parser<'a, &'a str, O, Extra<'a>> {}

impl<'a, T, O> P<'a, O> for T where T: Parser<'a, &'a str, O, Extra<'a>> {}

pub fn parse_dec_or_hex(s: &str) -> Result<u32> {
    if let Some(s) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        Ok(u32::from_str_radix(s, 16)?)
    } else {
        Ok(s.parse::<u32>()?)
    }
}

pub fn print_errors<E>(errs: &[Rich<'_, E>], input: &str) -> Result<()>
where
    E: Display,
{
    for err in errs {
        let span = err.span().into_range();

        Report::build(ReportKind::Error, span.clone())
            .with_message("Failed to parse")
            .with_label(Label::new(span).with_message(err.reason()))
            .finish()
            .eprint(Source::from(input))?;
    }

    Ok(())
}
