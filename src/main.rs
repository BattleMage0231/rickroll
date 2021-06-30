use rickroll::lexer::*;
use rickroll::parser::*;
use rickroll::interpreter::*;

use std::env::args;
use std::io::*;

fn main() {
    let raw = String::from("
    [Chorus]
    Never gonna let x down
    Never gonna let y down
    Never gonna give x 5
    Never gonna give y 6
    Never gonna let a down
    (Ooh give you a) Never gonna run ArrayOf and desert x, y
    Never gonna say a
    Never gonna let i down
    Never gonna give i 0
    Inside we both know i <= 10
        Never gonna let length down
        (Ooh give you length) Never gonna run ArrayLength and desert a
        (Ooh give you a) Never gonna run ArrayPush and desert a, length, x
        Never gonna give i i + 1
    We know the game and we're gonna play it
    Never gonna say a
    Inside we both know TRUE
        Never gonna give i 0
        (Ooh give you a) Never gonna run ArrayPop and desert a, i
        Never gonna say a
    We know the game and we're gonna play it
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
