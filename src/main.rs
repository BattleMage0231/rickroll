use rickroll::lexer::*;
use rickroll::parser::*;
use rickroll::interpreter::*;

use std::env::args;
use std::io::*;

fn main() {
    let mut arguments = args();
    arguments.next();
    let raw = format!(
        "\
[Verse fib]
(Ooh give you a)
Inside we both know a <= 1
  (Ooh) Never gonna give, never gonna give (give you a)
Your heart's been aching but you're too shy to say it
Never gonna let b down
Never gonna let c down
Never gonna give b a - 1
Never gonna give c a - 2
(Ooh give you b) Never gonna run fib and desert b
(Ooh give you c) Never gonna run fib and desert c
(Ooh) Never gonna give, never gonna give (give you b + c)

[Chorus]
Never gonna let a down
Never gonna give a {}
(Ooh give you a) Never gonna run fib and desert a
Never gonna say a
    ",
        arguments.next().unwrap()
    );
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
