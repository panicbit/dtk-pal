use chumsky::prelude::*;
use chumsky::text::newline;

use crate::symbols::{Attrs, ObjDataKind, ObjSymbol, ObjSymbolFlag, ObjSymbolKind};
use crate::util::parsing::{P, parse_dec_or_hex};

pub fn symbols<'a>() -> impl P<'a, Vec<ObjSymbol<'a>>> {
    symbol_line()
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(end())
}

fn symbol_line<'a>() -> impl P<'a, ObjSymbol<'a>> {
    name()
        .padded()
        .then_ignore(just('=').padded())
        .then(section_and_addr())
        .then_ignore(just(';'))
        .then(just("//").padded().ignore_then(attrs()).or_not())
        .then_ignore(newline().or(end()))
        .map(|((name, (section, addr)), attrs)| ObjSymbol {
            name,
            section,
            addr,
            attrs: attrs.unwrap_or_default(),
        })
}

fn name<'a>() -> impl P<'a, &'a str> {
    regex(r"[^\s=]+")
}

fn section_and_addr<'a>() -> impl P<'a, (Option<&'a str>, &'a str)> {
    section().then_ignore(just(':')).or_not().then(addr())
}

fn section<'a>() -> impl P<'a, &'a str> {
    regex(r"[A-Za-z0-9.]+")
}

fn addr<'a>() -> impl P<'a, &'a str> {
    regex(r"[0-9A-Fa-fXx]+")
}

fn attrs<'a>() -> impl P<'a, Attrs> {
    let attrs = key_value_attr().or(key_attr());

    attrs.separated_by(just(' ').repeated().at_least(1)).fold(
        Attrs::default(),
        |mut attrs, attr| {
            match attr {
                Attr::Type(obj_symbol_kind) => attrs.r#type = obj_symbol_kind,
                Attr::Flag(obj_symbol_flag) => attrs.flags |= obj_symbol_flag,
                Attr::Size(size) => attrs.size = Some(size),
                Attr::Data(obj_data_kind) => attrs.data = obj_data_kind,
                Attr::Hidden => attrs.hidden = true,
                Attr::Align(align) => attrs.align = align,
            }

            attrs
        },
    )
}

#[derive(Debug)]
enum Attr {
    Type(ObjSymbolKind),
    Flag(ObjSymbolFlag),
    Size(u32),
    Data(ObjDataKind),
    Align(u32),
    Hidden,
}

fn key_attr<'a>() -> impl P<'a, Attr> {
    any()
        .filter(|c| *c != ' ' && *c != '\n')
        .repeated()
        .at_least(1)
        .to_slice()
        .try_map(|key, span| {
            Ok(match key {
                "hidden" => Attr::Hidden,
                _ => return Err(Rich::custom(span, format!("unknown attribute: {key:?}"))),
            })
        })
}

fn key_value_attr<'a>() -> impl P<'a, Attr> {
    let name = any()
        .filter(|c| *c != ':' && *c != ' ' && *c != '\n')
        .repeated()
        .at_least(1)
        .to_slice();
    let value = any()
        .filter(|c| *c != ' ' && *c != '\n')
        .repeated()
        .at_least(1)
        .to_slice();
    let colon = just(':');
    let key_value = name.then_ignore(colon).then(value);

    key_value.try_map(|(key, value), span| {
        match key {
            "type" => ObjSymbolKind::from_str(value).map(Attr::Type),
            "scope" => ObjSymbolFlag::from_str(value).map(Attr::Flag),
            "size" => parse_dec_or_hex(value).map(Attr::Size),
            "data" => ObjDataKind::from_str(value).map(Attr::Data),
            "align" => parse_dec_or_hex(value).map(Attr::Align),
            _ => Err(eyre::eyre!("unknown attribute: {key:?}")),
        }
        .map_err(|err| Rich::custom(span, err))
    })
}
