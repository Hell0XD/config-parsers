
use crate::parsers::*;
use crate::types::*;


use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Object(HashMap<&'a str, JsonValue<'a>>),
    Array(Vec<JsonValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn null<'a>() -> impl Parser<'a, ()> {
    match_literal("null")
}

fn array<'a>() -> impl Parser<'a, Vec<JsonValue<'a>>> {
    |input: &'a str| 
        map(right(
            match_literal("["), 
            left(
                pair(
                    zero_or_more(left(skip_whitespace_left(value()), skip_whitespace_left(match_literal(",")))),
                    skip_whitespace_left(value())),
                skip_whitespace_left(match_literal("]")))),
        |(mut vec, last)| {vec.push(last); vec}).parse(input)    
}


fn value<'a>() -> impl Parser<'a, JsonValue<'a>> {
    or(
        map(float(), |f| JsonValue::Number(f)),
        or(
            map(number(), |n| JsonValue::Number(n as f64)),
            or(
                map(string(), |s| JsonValue::String(s)),
                or(
                    map(null(), |_| JsonValue::Null),
                    or(
                        map(array(), |arr| JsonValue::Array(arr)),
                        or(
                            map(boolean(), |b| JsonValue::Boolean(b)),
                            map(object(), |obj| JsonValue::Object(obj))
                        )
                    )
                )
            )
        )
    )
}

fn object<'a>() -> impl Parser<'a, HashMap<&'a str, JsonValue<'a>>> {
    |input: &'a str| map(right(
        match_literal("{"), 
        left(
            pair(zero_or_more(pair(
                skip_whitespace_left(string()),
                right(
                    skip_whitespace_left(match_literal(":")),
                    left(skip_whitespace_left(value()), skip_whitespace_left(match_literal(","))))
                )), pair(skip_whitespace_left(string()), right(skip_whitespace_left(match_literal(":")), skip_whitespace_left(value())))), 
            skip_whitespace_left(match_literal("}")))),
        |(v, (last_key, last_value))| {
            let mut m = HashMap::with_capacity(v.len());

            v.into_iter().for_each(|(key, value)| {m.insert(key, value);});
            m.insert(last_key, last_value);

            m
        }).parse(input)
}


pub fn parse(input: &str) -> Result<JsonValue, &'static str> {
    skip_whitespace_left(value()).parse(input).map(|(_, result)| result)
}


