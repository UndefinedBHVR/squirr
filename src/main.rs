use ariadne::{sources, Color, Label, Report, ReportKind};
use ast::lexer;
use chumsky::Parser;

mod ast;
fn main() {
    use chumsky::prelude::*;
    let filename = "test";
    let src = r#"
    0.1111.1
    16
    0x0CA
    // This is a single line comment
    # This is also a single line comment
    /* This is an unfinished block comment.
    When this fails, this should span to the end of this code.:
    Example text that fails, I guess
    */
    "#;
    let _x = lexer();
    let (tokens, errs) = lexer().parse(src).into_output_errors();
    let _t = 0x0C;
    if let Some(tokens) = tokens {
        tokens
            .iter()
            .for_each(|(token, _span)| println!("{token:?}"));
    }
    errs.into_iter()
        //.map(|err_base| err_base.map_token(|c| c.to_string()))
        .for_each(|e| {
            Report::build(ReportKind::Error, filename.clone(), e.span().start)
                .with_code(e.code)
                .with_message(e.to_string())
                .with_label(
                    Label::new((filename.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .eprint(sources([(filename.clone(), src.clone())]))
                .unwrap()
        });
}
