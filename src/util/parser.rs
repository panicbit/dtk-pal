use chumsky::prelude::*;

use crate::util::parsing::P;

pub fn take_while1<'a, F>(f: F) -> impl P<'a, &'a str>
where
    F: Fn(char) -> bool,
{
    any()
        .filter(move |c| f(*c))
        .repeated()
        .at_least(1)
        .to_slice()
}

pub fn take_until_space1<'a>() -> impl P<'a, &'a str> {
    take_while1(|c| !c.is_ascii_whitespace())
}
