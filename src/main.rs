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

mod lexer;

fn main() {
    let source = r#"
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
      -> out_type []"#;

    let tokens = lexer::lex(&source);

    for (token, slice) in tokens {
        println!("{:?} -> {}", token, slice);
    }
}
