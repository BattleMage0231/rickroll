use rickroll::lexer::Lexer;
use rickroll::parser::Parser;
use rickroll::interpreter::Interpreter;

use std::fs::File;
use std::io::*;
use std::path::PathBuf;

use structopt::StructOpt;
use ansi_term::Colour::Red;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, about="Print debugging information")]
    debug: bool,
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn execute(file: PathBuf, debug: bool) -> std::result::Result<(), Error> {
    // read from file
    let mut f = File::open(file)?;
    let mut raw = String::new();
    f.read_to_string(&mut raw)?;
    if debug {
        eprintln!("{}", Red.paint("Started lexing..."));
    }
    let lexer = Lexer::new(raw);
    let tokens = lexer.parse();
    match tokens {
        Err(e) => {
            eprintln!("{}", Red.paint(format!("{}", e)));
            return Ok(());
        }
        _ => (),
    };
    let tokens = tokens.unwrap();
    if debug {
        println!("{:?}", tokens);
        eprintln!("{}", Red.paint("Finished lexing..."));
        eprintln!("{}", Red.paint("Started parsing..."));
    }
    let parser = Parser::new(tokens);
    let parsed = parser.parse();
    match parsed {
        Err(e) => {
            eprintln!("{}", Red.paint(format!("{}", e)));
            return Ok(());
        }
        _ => (),
    };
    let parsed = parsed.unwrap();
    if debug {
        println!("{:?}", parsed);
        eprintln!("{}", Red.paint("Finished parsing..."));
        eprintln!("{}", Red.paint("Started executing..."));
    }
    let mut interpreter = Interpreter::new(parsed);
    let result = interpreter.run(&mut stdout(), &mut BufReader::new(stdin()));
    match result {
        Err(e) => {
            eprintln!("{}", Red.paint(format!("{}", e)));
            return Ok(());
        }
        _ => (),
    }
    if debug {
        println!(
            "\n{:#?}",
            result.unwrap()
        );
    }
    return Ok(());
}

fn main() -> std::result::Result<(), Error> {
    let opt = Opt::from_args();
    execute(opt.file, opt.debug)?;
    return Ok(());
}
