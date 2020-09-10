use rickroll::compiler::*;
use rickroll::interpreter::*;

use std::io::*;

fn main() {
    let src = "\
    Never gonna let a down
    Never gonna give a 3
    Never gonna let b down
    Never gonna say a
    Never gonna say b
    Never gonna say a + 3
    "
    .to_string();
    let compiled = Compiler::new(src).compile().unwrap();
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
}
