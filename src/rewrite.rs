use winnow::Parser;
use winnow::ascii::alphanumeric0;
use winnow::combinator::{delimited, preceded};
use winnow::prelude::*;
use winnow::stream::Stream;
use winnow::token::{tag, take_while};

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'src> {
    Identifier(&'src str),
    Null,
    IntegerLiteral(i64),
    IntBase16(i64),
    IntBase8(i64),
    Char(&'src str),
    Float(f64),
    String(&'src str),
    StringVerbatim(&'src str),
    Comment(Comment<'src>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comment<'src> {
    Single(&'src str),
    Multi(&'src str),
}

// Winnow parser for parsing the core of the squirrel language
pub fn squirr(input: &mut str) -> PResult<String> {

    todo!()
}

// Winnow parser for hexadecimals (base16)
pub fn int16<'s>(input: &mut &'s str) -> PResult<Token<'s>> {
    "0x".parse_next(input)?;
    let digits = take_while(1.., (
        ('0'..='9'),
        ('A'..='F'),
        ('a'..='f'),
    )).parse_next(input)?;
    // parse the digits as base16 hexadecimal
    Ok(Token::IntBase16(i64::from_str_radix(digits, 16).unwrap()))
}

// Winnow parser for octals (base8)
pub fn int8<'s>(input: &mut &'s str) -> PResult<Token<'s>> {
    "0".parse_next(input)?;
    let digits = take_while(1.., ('0'..='7')).parse_next(input)?;
    // parse the digits as base8 octal
    Ok(Token::IntBase8(i64::from_str_radix(digits, 8).unwrap()))
}

// Winnow parser for integers
pub fn int<'s>(input: &mut &'s str) -> PResult<Token<'s>> {
    let digits = take_while(1.., ('0'..='9')).parse_next(input)?;
    // parse the digits as base10 decimal
    Ok(Token::IntegerLiteral(i64::from_str_radix(digits, 10).unwrap()))
}

// Winnow parser for floats
pub fn float<'s>(input: &mut &'s str) -> PResult<Token<'s>> {
    let digits = take_while(1.., ('0'..='9')).parse_next(input)?;
    let _ = ".".parse_next(input)?;
    let digits2 = take_while(1.., ('0'..='9')).parse_next(input)?;
    // parse the digits as base10 decimal
    Ok(Token::Float(format!("{}.{}", digits, digits2).parse::<f64>().unwrap()))
}

mod test {
    // Test hexadecimals parsing
    #[test]
    fn test_int16() {
        let mut input = "0x0CA";
        let result = super::int16(&mut input);
        assert_eq!(result, Ok(super::Token::IntBase16(202)));
    }

    // Test octals parsing (077)
    #[test]
    fn test_int8() {
        let mut input = "077";
        let result = super::int8(&mut input);
        assert_eq!(result, Ok(super::Token::IntBase8(63)));
    }

    // Test integers parsing
    #[test]
    fn test_int() {
        let mut input = "123456789";
        let result = super::int(&mut input);
        assert_eq!(result, Ok(super::Token::IntegerLiteral(123456789)));
    }

    // Test floats parsing
    #[test]
    fn test_float() {
        let mut input = "123.456";
        let result = super::float(&mut input);
        assert_eq!(result, Ok(super::Token::Float(123.456)));
    }

}