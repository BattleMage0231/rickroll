use rickroll::interpreter::*;

use rickroll::util::{
    Instruction::{self, *},
    Operator::*,
    RickrollObject::*,
    Token::*,
};

// helper functions
fn get(s: Vec<(usize, Instruction)>, stdout: Vec<u8>, stdin: String) -> String {
    let mut stdout = stdout;
    use std::io::BufReader;
    let res = Interpreter::new(s).run(&mut stdout, &mut BufReader::new(stdin.as_bytes()));
    return match res {
        Ok(_) => String::from_utf8(stdout).unwrap(),
        Err(err) => String::from(format!("{:?}", err)),
    };
}

fn assert_eqv(raw: Vec<(usize, Instruction)>, stdin: &str, res: &str) {
    assert_eq!(&get(raw, Vec::new(), String::from(stdin))[..], res);
}

#[test]
fn simple() {
    assert_eqv(
        vec![
            (1, Put(vec![Value(Int(1)), Operator(Add), Value(Int(2))])),
            (
                2,
                Put(vec![Value(Int(3)), Operator(Greater), Value(Int(4))]),
            ),
            (0, End()),
        ],
        "",
        "3\nFALSE\n",
    );
    assert_eqv(
        vec![
            (1, Let("a".to_string())),
            (2, Set("a".to_string(), vec![Value(Int(3))])),
            (3, Let("b".to_string())),
            (4, Put(vec![Variable("a".to_string())])),
            (5, Put(vec![Variable("b".to_string())])),
            (
                6,
                Put(vec![
                    Variable("a".to_string()),
                    Operator(Add),
                    Value(Int(3)),
                ]),
            ),
            (0, End()),
        ],
        "",
        "3\nUNDEFINED\n6\n",
    );
}

// should err
#[test]
fn error() {
    assert_eqv(
        vec![
            (1, Put(vec![Value(Int(3)), Operator(And)]))
        ],
        "",
        "Error { err: Traceback, desc: \"\", line: Some(1), child: Some(Error { err: IllegalArgumentError, desc: \"Not enough arguments\", line: None, child: None }) }",
    );
}
