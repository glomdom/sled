use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind};
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    UnrecognizedToken,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = LexicalError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("|")]
    Pipe,

    #[token("mold")]
    Mold,

    #[token("claim")]
    Claim,

    #[token("->")]
    LeftPointer,
    #[token("<-")]
    RightPointer,

    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("==")]
    Equal,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<=")]
    LessThanOrEqual,

    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    MultiplyEqual,
    #[token("/=")]
    DivideEqual,

    #[token("::")]
    DoubleColon,

    #[token("?")]
    QuestionMark,

    #[token("=")]
    Assign,

    #[token("whence")]
    Whence,
    #[token("orwhence")]
    OrWhence,
    #[token("other")]
    Other,

    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,

    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("forin")]
    ForIn,

    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice().to_owned())]
    String(String),

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

pub fn lex(input: &str) -> Result<Vec<(Token, Range<usize>, &str)>, Report> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let span = lexer.span();

        match token {
            Ok(Token::String(_)) | Ok(Token::Identifier) | Ok(Token::Number) => {
                tokens.push((token.unwrap(), span.clone(), lexer.slice()));
            }

            Ok(token) => {
                tokens.push((token, span.clone(), lexer.slice()));
            }

            Err(LexicalError::UnrecognizedToken) => {
                let report = Report::build(ReportKind::Error, (), span.start)
                    .with_message("unrecognized token")
                    .with_label(
                        Label::new(span)
                            .with_message("unexpected token")
                            .with_color(Color::Red),
                    )
                    .finish();

                return Err(report);
            }
        }
    }

    Ok(tokens)
}
