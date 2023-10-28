// use std::ops::Deref;

// use chumsky::{
//     error::{RichPattern, RichReason},
//     prelude::*,
//     util::MaybeRef,
// };
// pub type Span = SimpleSpan<usize>;

// #[derive(Debug)]
// pub enum Token<'src> {
//     Identifier(&'src str),
//     Null,
//     IntegerLiteral(i64),
//     IntBase16(i64),
//     IntBase8(i64),
//     Char(&'src str),
//     Float(f64),
//     String(&'src str),
//     StringVerbatim(&'src str),
//     Comment(Comment<'src>),
// }

// #[derive(Debug)]
// pub enum Comment<'src> {
//     Single(&'src str),
//     Multi(&'src str),
// }

// pub struct Info<'a, T, S = SimpleSpan<usize>, L = &'static str> {
//     span: S,
//     reason: Box<RichReason<'a, T, L>>,
//     context: Vec<(L, S)>,
// }

// impl<'a, T, S> Deref for Info<'a, T, S> {
//     type Target = Rich<'a, T, S>;

//     fn deref(&self) -> &Self::Target {
//         unsafe { std::mem::transmute(self) }
//     }
// }
// pub struct Error<'src> {
//     pub rich: Info<'src, char, Span>,
//     pub code: &'src str,
// }

// impl<'a> Error<'a> {
//     pub fn replace_reason(&mut self, reason: RichReason<'a, char>) {
//         self.rich.reason = Box::new(reason);
//     }
// }
// impl<'a> chumsky::error::Error<'a, &'a str> for Error<'a> {
//     fn expected_found<Iter: IntoIterator<Item = Option<MaybeRef<'a, char>>>>(
//         expected: Iter,
//         found: Option<MaybeRef<'a, char>>,
//         span: Span,
//     ) -> Self {
//         let reason = RichReason::ExpectedFound {
//             expected: expected
//                 .into_iter()
//                 .map(|tok| {
//                     tok.map(RichPattern::<char, &'static str>::Token)
//                         .unwrap_or(RichPattern::EndOfInput)
//                 })
//                 .collect(),
//             found,
//         };
//         // Note: This is safe, these types are identical. There's not a way to create a Rich by hand, so I need this bad hack.
//         let rich = Info {
//             span,
//             reason: Box::new(reason),
//             context: Vec::new(),
//         };
//         Self { rich, code: "0001" }
//     }
// }

// impl<'src> Deref for Error<'src> {
//     type Target = Rich<'src, char, Span>;

//     fn deref(&self) -> &Self::Target {
//         &self.rich
//     }
// }

// pub fn lexer<'src>(
// ) -> impl Parser<'src, &'src str, Vec<(Token<'src>, Span)>, extra::Err<Error<'src>>> {
//     // Base_16 sats with a 0x
//     let base_16 = just("0x").ignore_then(
//         text::digits(16)
//         .collect::<String>()
//         .map(|str| i64::from_str_radix(&str, 16).unwrap())
//         .map(Token::IntBase16)
//     );

//     let base_8 = just("0")
//         .then(text::digits(8))
//         .to_slice()
//         .from_str()
//         .unwrapped()
//         .map(Token::IntegerLiteral);

//     let integer_literal = one_of("123456789")
//         .then(text::digits(10))
//         .to_slice()
//         .from_str()
//         .unwrapped()
//         .map(Token::IntegerLiteral)
//         .or(base_16)
//         .or(base_8);

//     let float = text::int(10)
//         .then(just("."))
//         .then(text::digits(10))
//         .to_slice()
//         .from_str()
//         .unwrapped()
//         .map(Token::Float)
//         .map_err_with_state(|mut err: Error<'src>, _span: Span, _| match err.reason() {
//             _ if err.reason().found().is_some()
//                 && format!("{}", err.reason()).ends_with("expected something else") =>
//             {
//                 let reason = RichReason::Custom(format!(
//                     "invalid character in float: {:?}",
//                     err.reason().found().unwrap()
//                 ));
//                 err.replace_reason(reason);
//                 err
//             }
//             _ => err,
//         });
//     let single_line_comment = just("//")
//         .or(just("#"))
//         .then(any().and_is(just('\n').not()).repeated())
//         .padded()
//         .to_slice()
//         .map(Comment::Single)
//         .map(Token::Comment);

//     let multi_line_comment = any()
//         .and_is(just("*/").not())
//         .repeated()
//         .delimited_by(just("/*"), just("*/"))
//         .padded()
//         .to_slice()
//         .map(Comment::Multi)
//         .map_err_with_state(|mut err: Error<'src>, _span: Span, _| match err.reason() {
//             _ if format!("{}", err.reason()).starts_with("found end of input expected '*") => {
//                 let reason =
//                     RichReason::Custom("missing trailing `*/` for block comment".to_string());
//                 err.replace_reason(reason);
//                 err
//             }
//             _ => err,
//         })
//         .map(Token::Comment);

//     integer_literal
//         .or(single_line_comment)
//         .or(multi_line_comment)
//         .or(float)
//         .map_with(|tok, span| (tok, span.span()))
//         .padded()
//         // If we encounter an error, skip and attempt to lex the next character as a token instead
//         .recover_with(skip_then_retry_until(any().ignored(), end()))
//         .repeated()
//         .collect()
// }
