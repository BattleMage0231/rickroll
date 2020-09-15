use rickroll::compiler::*;
use rickroll::interpreter::*;

use std::io::*;

fn main() {
    /*
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
    let compiled = Compiler::new(src).compile();
    let compiled = match compiled {
        Err(err) => panic!("{}", err),
        Ok(val) => val,
    };
    println!("{:?}", compiled);
    Interpreter::new(compiled)
        .run(&mut stdout(), &mut BufReader::new(&mut stdin()))
        .unwrap();
    /*
    use std::io::*;
    writeln!(&mut stdout(), "Test");
    writeln!(&mut stdout(), "Test2");
    use std::time::Duration;
    use std::thread;
    thread::sleep(Duration::from_secs(4));
    writeln!(&mut stdout(), "Test3");
    */
    */
}
