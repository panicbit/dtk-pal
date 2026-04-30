use chumsky::prelude::*;
use chumsky::text::whitespace;

use crate::splits::{File, Section, Split, Splits};
use crate::util::parser::{take_until_space1, take_while1};
use crate::util::parsing::{P, parse_dec_or_hex};

pub fn splits<'a>() -> impl P<'a, Splits> {
    let files = file().separated_by(whitespace()).collect::<Vec<_>>();

    sections()
        .then_ignore(whitespace())
        .then(files)
        .then_ignore(whitespace())
        .map(|(sections, files)| Splits { sections, files })
}

pub fn sections<'a>() -> impl P<'a, Vec<Section>> {
    let sections = section().separated_by(just('\n')).collect::<Vec<_>>();

    just("Sections:\n").ignore_then(sections)
}

pub fn section<'a>() -> impl P<'a, Section> {
    let name = take_until_space1();
    let whitespace = just(' ').repeated().at_least(1);
    let r#type = just("type:").ignore_then(take_until_space1());
    let align = just("align:")
        .ignore_then(take_until_space1())
        .try_map(|align, span| parse_dec_or_hex(align).map_err(|err| Rich::custom(span, err)));

    just('\t')
        .ignore_then(name)
        .then_ignore(whitespace)
        .then(r#type)
        .then_ignore(just(' '))
        .then(align)
        .map(|((name, r#type), align)| Section {
            name: name.into(),
            r#type: r#type.into(),
            align,
        })
}

pub fn file<'a>() -> impl P<'a, File> {
    let path = take_while1(|c| c != ':' && !c.is_ascii_whitespace());
    let splits = split().separated_by(just('\n')).collect::<Vec<_>>();

    path.then_ignore(just(":\n"))
        .then(splits)
        .map(|(path, splits)| File {
            path: path.into(),
            splits,
        })
}

pub fn split<'a>() -> impl P<'a, Split> {
    let section = take_until_space1();
    let whitespace = just(' ').repeated().at_least(1);
    let start = just("start:")
        .ignore_then(take_until_space1())
        .try_map(|align, span| parse_dec_or_hex(align).map_err(|err| Rich::custom(span, err)));
    let end = just("end:")
        .ignore_then(take_until_space1())
        .try_map(|align, span| parse_dec_or_hex(align).map_err(|err| Rich::custom(span, err)));

    just('\t')
        .ignore_then(section)
        .then_ignore(whitespace)
        .then(start)
        .then_ignore(just(' '))
        .then(end)
        .map(|((section, start), end)| Split {
            section: section.into(),
            start,
            end,
        })
}
