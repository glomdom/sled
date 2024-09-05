use std::ops::Range;

use ariadne::{Color, Label, Report, ReportKind};
use logos::Logos;
use thiserror::Error;

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

    #[regex(r#""[^"]*"#, priority = 1)]
    UnclosedString,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

// Function to run the lexer and handle unclosed strings
pub fn lex(input: &str) -> Result<Vec<(Token, Range<usize>, &str)>, Report> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        let span = lexer.span();

        match token {
            Ok(Token::UnclosedString) => {
                let report = Report::build(ReportKind::Error, (), span.start)
                    .with_message("unclosed string literal")
                    .with_label(Label::new(span)
                        .with_message("string starting here was not closed")
                        .with_color(Color::Red))
                    .finish();

                return Err(report);
            }

            Ok(Token::String(_)) | Ok(Token::Identifier) | Ok(Token::Number) => {
                tokens.push((token.unwrap(), span.clone(), lexer.slice()));
            }

            Ok(token) => {
                tokens.push((token, span.clone(), lexer.slice()));
            }

            Err(LexicalError::UnrecognizedToken) => {

            }
        }
    }

    Ok(tokens)
}
