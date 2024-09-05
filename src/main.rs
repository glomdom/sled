/*

    sled - an obscure systems(?) programming language
    Copyright (C) 2024  glomdom

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

*/

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
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

    #[regex(r#""([^"\\]|\\.)*""#)]
    String,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

fn main() {
    let mut lex = Token::lexer(
        r#"
    mold Glomdom [
      name :: string
      age :: integer
      programmer :: theory
    ]

    claim x = 2
    claim y = whence x == 2 [ 10 ] other [ 5 ]

    whence x > 10 [
      print "massiv balls"
    ] orwhence x == 10 [
      print "latinas"
    ] other [
      print "black"
    ]

    | fnname
      <- in1 :: type ? default_value
      <- in2 :: type
      -> out_type []"#,
    );

    while let Some(token) = lex.next() {
        println!("{:?}: {}", token, lex.slice());
    }
}
