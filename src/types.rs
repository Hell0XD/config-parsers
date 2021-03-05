use crate::parsers::*;


pub fn string<'a>() -> impl Parser<'a, &'a str> {
    right(match_literal("\""), left(null_identifier(|ch| ch != '"'), match_literal("\"")))
}

fn _number<'a>() -> impl Parser<'a, u32> {
    map(identifier(|ch| ch.is_numeric()), |s| s.parse().unwrap())
}

pub fn number<'a>() -> impl Parser<'a, i32> {
    or(map(right(match_literal("-"), _number()), |n| n as i32 * -1), map(_number(), |n| n as i32))
}

fn _float<'a>() -> impl Parser<'a, f64> {
    map(pair(_number(), right(match_literal("."), _number())), 
        |(num1, num2)| num1 as f64 + num2 as f64 / 10_i32.pow((num2 as f64).log10().ceil() as u32) as f64)
}

pub fn float<'a>() -> impl Parser<'a, f64> {
    or(map(right(match_literal("-"), _float()), |f| f * -1.0), _float())
}

pub fn boolean<'a>() -> impl Parser<'a, bool> {
    or(map(match_literal("true"), |_| true), map(match_literal("false"), |_| false))
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        assert_eq!(Ok(("", "fizz")), string().parse("\"fizz\""))
    }
}