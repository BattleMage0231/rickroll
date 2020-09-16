use rickroll::compiler::*;
use rickroll::interpreter::*;
use rickroll::lexer::*;

use std::io::*;

fn main() {
    let src ="\
    Never gonna let a down
    Never gonna give a 5
    Inside we both know a == 5
        Never gonna say TRUE
    Your heart's been aching but you're too shy to say it
    "
    .to_string();
    println!("Started lexing");
    let lexed = Lexer::new(src).parse().unwrap();
    println!("Finished lexing");
    println!("Result:\n{:?}\n", lexed.to_vec());
    println!("Started compiling");
    let compiled = Compiler::new(lexed).compile();
    println!("Finished compiling");
    println!("Result:\n{:?}\n", compiled);
}
