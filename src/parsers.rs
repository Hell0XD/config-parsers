
pub type ParserError = &'static str;
pub type ParserResult<'a, R> = Result<(&'a str, R), ParserError>;

pub trait Parser<'a, R> {
    fn parse(&self, input: &'a str) -> ParserResult<'a, R>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParserResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParserResult<'a, Output> {
        self(input)
    }
}


pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| 
        if input.starts_with(expected) {
            Ok((&input[expected.len()..], ()))
        }else{
            Err("match literal failed")
        }
}

pub fn null_identifier<'a>(valid_char: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        let mut index = 0;
        let mut chars = input.chars();

        while let Some(next) = chars.next() {
            if !valid_char(next) {
                break;
            }

            index += 1;
        }

        return Ok((&input[index..], &input[0..index]));
    }
}

pub fn identifier_first<'a>(valid_char: impl Fn(char) -> bool, first: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        if let Some(next) = input.chars().next() {
            if !first(next) {
                return Err("identifier failed");
            }
        }else{
            return Err("identifier failed");
        }

        let mut index = 0;
        let mut chars = input.chars();

        while let Some(next) = chars.next() {
            if !valid_char(next) {
                break;
            }

            index += 1;
        }

        return Ok((&input[index..], &input[0..index]));
    }  
}

pub fn identifier<'a>(valid_char: impl Fn(char) -> bool) -> impl Parser<'a, &'a str> {
    move |input: &'a str| {
        if let Some(next) = input.chars().next() {
            if !valid_char(next) {
                return Err("identifier failed");
            }
        }else{
            return Err("identifier failed");
        }

        let mut index = 0;
        let mut chars = input.chars();

        while let Some(next) = chars.next() {
            if !valid_char(next) {
                break;
            }

            index += 1;
        }

        return Ok((&input[index..], &input[0..index]));
    }  
}

pub fn zero_or_more<'a, A>(parser: impl Parser<'a, A>) -> impl Parser<'a, Vec<A>> {
    move |mut input: &'a str| {
        let mut vec = Vec::new();

        while let Ok((_input, val)) = parser.parse(input) {
            input = _input;
            vec.push(val);
        }

        return Ok((input, vec));
    }
}

pub fn one_or_more<'a, A>(parser: impl Parser<'a, A>) -> impl Parser<'a, Vec<A>> {
    move |input: &'a str| {
        let (input, val) = parser.parse(input)?;

        let (input, mut vec) = zero_or_more(|input| parser.parse(input)).parse(input)?;
        vec.insert(0, val);

        return Ok((input, vec));
    }
}

pub fn map<'a, A, B>(parser: impl Parser<'a, A>, f: impl Fn(A) -> B) -> impl Parser<'a, B> {
    move |input: &'a str| parser.parse(input).map(|(input, val)| (input, f(val)))
}

pub fn and_then<'a, A, B>(parser: impl Parser<'a, A>, f: impl Fn(A) -> Result<B, ParserError>) -> impl Parser<'a, B> {
    move |input: &'a str| parser.parse(input).and_then(|(input, val)| Ok((input, f(val)?)))
}

pub fn pair<'a, A, B>(parser1: impl Parser<'a, A>, parser2: impl Parser<'a, B>) -> impl Parser<'a, (A, B)> {
    move |input: &'a str| {
        parser1.parse(input)
            .and_then(|(input, val1)| parser2.parse(input)
                .map(move |(input, val2)| (input, (val1, val2))))
    }
}

pub fn left<'a, A, B>(parser1: impl Parser<'a, A>, parser2: impl Parser<'a, B>) -> impl Parser<'a, A> {
    map(pair(parser1, parser2), |(left, _)| left)
}

pub fn right<'a, A, B>(parser1: impl Parser<'a, A>, parser2: impl Parser<'a, B>) -> impl Parser<'a, B> {
    map(pair(parser1, parser2), |(_, right)| right)
}

pub fn or<'a, A>(parser1: impl Parser<'a, A>, parser2: impl Parser<'a, A>) -> impl Parser<'a, A> {
    move |input: &'a str| parser1.parse(input).or_else(|_| parser2.parse(input))
}


pub fn maybe<'a, A>(parser: impl Parser<'a, A>, default: impl Fn() -> A) -> impl Parser<'a, A> {
    move |input: &'a str| parser.parse(input).or_else(|_| Ok((input, default())))
}


pub fn skip_whitespace<'a>() -> impl Parser<'a, ()> {
    map(null_identifier(|ch| ch.is_whitespace()), |_| ())
}


pub fn skip_whitespace_left<'a, A>(parser: impl Parser<'a, A>) -> impl Parser<'a, A> {
    right(skip_whitespace(), parser)
}




#[cfg(test)]
mod test {
    use crate::parsers::*;

    #[test]
    fn test_match_literal() {
        let foo_parser = match_literal("foo");

        assert_eq!(Ok(("bar", ())), foo_parser.parse("foobar"));
    }

    #[test]
    fn test_identifier() {
        let xml_identifier = identifier(|char| char.is_alphanumeric());

        assert_eq!(Ok(("", "foo")), xml_identifier.parse("foo"));
    }


    #[test]
    fn test_one_or_more() {
        let bar_more_parser = one_or_more(match_literal("bar"));

        assert_eq!(Ok(("foo", vec![(), (), ()])), bar_more_parser.parse("barbarbarfoo"));
    }
}
