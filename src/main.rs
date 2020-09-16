use rickroll::compiler::*;
use rickroll::interpreter::*;
use rickroll::lexer::*;

use std::io::*;

fn main() {
    let src = "\
    Never gonna let n down
    Never gonna give n 10
    Never gonna let first down
    Never gonna let second down
    Never gonna give first 0
    Never gonna give second 1
    Never gonna say second
    Inside we both know n != 0
        Never gonna let sum down
        Never gonna give sum first + second
        Never gonna say sum
        Never gonna give first second
        Never gonna give second sum
        Never gonna give n n - 1
    We know the game and we're gonna play it
    "
    .to_string();
    println!("Started lexing");
    let lexed = Lexer::new(src).parse().unwrap();
    println!("Finished lexing");
    println!("Started compiling");
    let compiled = Compiler::new(lexed).compile();
    println!("Finished compiling");
    let compiled = match compiled {
        Err(err) => panic!("{}", err),
        Ok(val) => val,
    };
    println!("{:?}", compiled);
    Interpreter::new(compiled)
        .run(&mut stdout(), &mut BufReader::new(&mut stdin()))
        .unwrap();
}
