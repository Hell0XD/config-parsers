

use crate::parsers::*;
use crate::types::string;

use std::collections::HashMap;



fn key_pair<'a>() -> impl Parser<'a, (&'a str, &'a str)> {
    pair(
        identifier(|ch| !(ch == '=' || ch == '\n')),
        right(skip_whitespace_left(match_literal("=")), or(skip_whitespace_left(string()), skip_whitespace_left(identifier(|ch| ch != '\n'))))
    )
}

pub fn parser<'a>(input: &'a str) -> Result<HashMap<&'a str, &'a str>, ParserError> {
    map(zero_or_more(skip_whitespace_left(key_pair())), |vals| {
        let mut m = HashMap::with_capacity(vals.len());

        vals.into_iter().for_each(|(key, val)| {m.insert(key, val);});

        m
    }).parse(input).map(|(_, vals)| vals)
}