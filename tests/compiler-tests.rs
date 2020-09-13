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
        "Ok(Bytecode { instructions: [Put([Value(Int(1)), Operator(Add), Value(Int(2))]), Put([Value(Int(3)), Operator(Greater), Value(Int(4))]), End], debug_lines: [1, 2, 0], alloc_stack: [] })",
    );
    assert_eqv(
        "\
        Never gonna let a down
        Never gonna give a 3
        Never gonna let b down
        Never gonna say a
        Never gonna say b
        Never gonna say a + 3
        ",
        "Ok(Bytecode { instructions: [Let(\"a\"), Set(\"a\", [Value(Int(3))]), Let(\"b\"), Put([Variable(\"a\")]), Put([Variable(\"b\")]), Put([Variable(\"a\"), Operator(Add), Value(Int(3))]), End], debug_lines: [1, 2, 3, 4, 5, 6, 0], alloc_stack: [] })",
    );
}

// while loops and if statements
#[test]
fn check() {
    assert_eqv(
        "\
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
        ",
        "Ok(Bytecode { instructions: [Let(\"n\"), Set(\"n\", [Value(Int(10))]), Let(\"first\"), Let(\"second\"), Set(\"first\", [Value(Int(0))]), Set(\"second\", [Value(Int(1))]), Put([Variable(\"second\")]), Jmpif([Variable(\"n\"), Operator(NotEquals), Value(Int(0))], 9), Jmp(18), Pctx, Let(\"sum\"), Set(\"sum\", [Variable(\"first\"), Operator(Add), Variable(\"second\")]), Put([Variable(\"sum\")]), Set(\"first\", [Variable(\"second\")]), Set(\"second\", [Variable(\"sum\")]), Set(\"n\", [Variable(\"n\"), Operator(Subtract), Value(Int(1))]), Dctx, Jmp(7), End], debug_lines: [1, 2, 3, 4, 5, 6, 7, 8, 8, 8, 9, 10, 11, 12, 13, 14, 15, 15, 0], alloc_stack: [] })",
    );
    assert_eqv(
        "\
        Never gonna let a down
        Never gonna give a 5
        Inside we both know a == 5
            Never gonna say TRUE
        Your heart's been aching but you're too shy to say it
        ",
        "Ok(Bytecode { instructions: [Let(\"a\"), Set(\"a\", [Value(Int(5))]), Jmpif([Variable(\"a\"), Operator(Equals), Value(Int(5))], 4), Jmp(7), Pctx, Put([Value(Bool(true))]), Dctx, End], debug_lines: [1, 2, 3, 3, 3, 4, 5, 0], alloc_stack: [] })",
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
    assert_eqv(
        "\
        Inside we both know TRUE
            Inside we both know TRUE
            Your heart's been aching but you're too shy to say it
        ",
        "Err(Error { err: SyntaxError, desc: \"Mismatched while or if start\", line: Some(1), child: None })",
    );
}
