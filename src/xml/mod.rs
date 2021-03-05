use crate::parsers::*;
use crate::types::*;



#[derive(Debug, PartialEq)]
pub enum XmlValue<'a> {
    Element(Element<'a>),
    Text(&'a str)
}
#[derive(Debug, PartialEq)]
pub struct Element<'a> {
    pub name: &'a str,
    pub attrs: Vec<(&'a str, &'a str)>,
    pub children: Vec<XmlValue<'a>>,
}

fn xml_valid_name_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn xml_valid_name(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == ':'
} 

fn attribute<'a>() -> impl Parser<'a, (&'a str, &'a str)> {
    pair(
        identifier_first(xml_valid_name, xml_valid_name_start),
        right(skip_whitespace_left(match_literal("=")), skip_whitespace_left(string())))
}

fn text<'a>() -> impl Parser<'a, &'a str> {
    identifier(|ch| ch != '<')
}

fn element_short<'a>() -> impl Parser<'a, Element<'a>> {
    map(right(
        match_literal("<"),
        pair(
            skip_whitespace_left(identifier_first(xml_valid_name, xml_valid_name_start)),
            left(zero_or_more(skip_whitespace_left(attribute())), pair(skip_whitespace_left(match_literal("/")), skip_whitespace_left(match_literal(">"))))
        )
    ), |(name, attrs)| Element{
        name,
        attrs,
        children: vec![]
    })
}

fn element_start<'a>() -> impl Parser<'a, Element<'a>> {
    map(right(
        match_literal("<"),
        pair(
            skip_whitespace_left(identifier_first(xml_valid_name, xml_valid_name_start)),
            left(zero_or_more(skip_whitespace_left(attribute())), skip_whitespace_left(match_literal(">")))
        )
    ), |(name, attrs)| Element{
        name,
        attrs,
        children: vec![]
    })
}

fn element_end<'a>() -> impl Parser<'a, &'a str> {
    right(
        pair(match_literal("<"), skip_whitespace_left(match_literal("/"))),
        left(
            skip_whitespace_left(identifier_first(xml_valid_name, xml_valid_name_start)),
            skip_whitespace_left(match_literal(">"))
        )
    )
}

fn element<'a>() -> impl Parser<'a, Element<'a>> {
    |input: &'a str| and_then(pair(
        element_start(), 
        pair(
            zero_or_more(or(map(skip_whitespace_left(text()), |t| XmlValue::Text(t)), map(skip_whitespace_left(or(element(), element_short())), |e| XmlValue::Element(e)))),
            skip_whitespace_left(element_end()))), |(mut el, (children, end))| {
                el.children = children;

                if el.name != end {
                    Err("ending tag does not match")
                }else{
                    Ok(el)
                }
            }).parse(input)
}

pub fn parse(input: &str) -> Result<Vec<Element>, &'static str> {
    zero_or_more(skip_whitespace_left(or(element(), element_short()))).parse(input).map(|(_, val)| val)
}
