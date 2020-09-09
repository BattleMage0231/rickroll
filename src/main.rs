use rickroll::compiler::*;

fn main() {
    let text = "        Never gonna say 1\nNever gonna say 7 * 3 >= 4 || TRUE\n\n";
    let compiled = Compiler::new(String::from(text)).compile();
    println!("{:?}", compiled);
    match compiled {
        Err(err) => println!("{}", err),
        _ => (),
    }
}
