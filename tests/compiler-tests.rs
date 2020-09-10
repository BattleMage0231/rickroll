use rickroll::compiler::*;

// helper functions
fn get(s: &str) -> String {
    String::from(format!("{:?}", Compiler::new(String::from(s)).compile()))
}

fn assert_eqv(raw: &str, res: &str) {
    assert_eq!(&get(raw)[..], res);
}

#[test]
fn simple() {
    assert_eqv(
        "\
        Never gonna say 1 + 2
        Never gonna say 3 > 4
        ",
        "Ok([(1, Put([Value(Int(1)), Operator(Add), Value(Int(2))])), (2, Put([Value(Int(3)), Operator(Greater), Value(Int(4))])), (0, End)])",
    );
    assert_eqv(
        "\
        Never gonna let a down
        Never gonna give a 1 + 2
        Never gonna let b down
        Never gonna say a
        Never gonna say b
        ",
        "Ok([(1, Let(\"a\")), (2, Set(\"a\", [Value(Int(1)), Operator(Add), Value(Int(2))])), (3, Let(\"b\")), (4, Put([Variable(\"a\")])), (5, Put([Variable(\"b\")])), (0, End)])",
    );
}

// should output Result::Error
#[test]
fn error() {
    assert_eqv(
        "\
        asdasdasdasd
        Never gonna say a
        ",
        "Err(Error { err: SyntaxError, desc: \"Illegal statement\", line: Some(1), child: None })",
    );
}
