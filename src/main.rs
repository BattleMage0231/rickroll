use rickroll::lexer::*;
use rickroll::parser::*;

fn main() {
    let expr = Lexer::new("1000000000.0 * 1000000000.0".to_string())
        .make_tokens()
        .unwrap();
    println!("{:?}", expr);
    let parsed = Parser::new(expr).eval();
    println!("{:?}", parsed);
}
