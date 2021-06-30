use rickroll::lexer::*;
use rickroll::parser::*;
use rickroll::interpreter::*;

use std::env::args;
use std::io::*;

fn main() {
    let raw = String::from("
    [Chorus]
    Never gonna let x down
    (Ooh give you x) Never gonna run ReadLine and desert you
    Never gonna say x
    (Ooh give you x) Never gonna run ReadLine and desert you
    Never gonna say x
    (Ooh give you x) Never gonna run ReadLine and desert you
    Never gonna say x
    ");
    println!("\"{}\"", raw);
    eprintln!("\x1b[0;31mStarted lexing...\x1b[0m");
    let lexer = Lexer::new(raw);
    let ir = lexer.parse().unwrap();
    println!("{:?}", ir);
    eprintln!("\x1b[0;31mFinished lexing...\x1b[0m");
    eprintln!("\x1b[0;31mStarted parsing...\x1b[0m");
    let parser = Parser::new(ir);
    let parsed = parser.parse().unwrap();
    println!("{:?}", parsed);
    eprintln!("\x1b[0;31mFinished parsing...\x1b[0m");
    eprintln!("\x1b[0;31mStarted interpreting...\x1b[0m");
    let mut interpreter = Interpreter::new(parsed);
    println!(
        "\n{:#?}",
        interpreter.run(&mut stdout(), &mut BufReader::new(stdin()))
    );
    eprintln!("\x1b[0;31mFinished interpreting...\x1b[0m");
}
