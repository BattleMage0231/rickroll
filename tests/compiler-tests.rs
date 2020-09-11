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
        Never gonna give a 3
        Never gonna let b down
        Never gonna say a
        Never gonna say b
        Never gonna say a + 3
        ",
        "Ok([(1, Let(\"a\")), (2, Set(\"a\", [Value(Int(3))])), (3, Let(\"b\")), (4, Put([Variable(\"a\")])), (5, Put([Variable(\"b\")])), (6, Put([Variable(\"a\"), Operator(Add), Value(Int(3))])), (0, End)])",
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
        "Ok([(1, Let(\"n\")), (2, Set(\"n\", [Value(Int(10))])), (3, Let(\"first\")), (4, Let(\"second\")), (5, Set(\"first\", [Value(Int(0))])), (6, Set(\"second\", [Value(Int(1))])), (7, Put([Variable(\"second\")])), (8, Jmpif([Variable(\"n\"), Operator(NotEquals), Value(Int(0))], 9)), (8, Jmp(18)), (8, Pctx), (9, Let(\"sum\")), (10, Set(\"sum\", [Variable(\"first\"), Operator(Add), Variable(\"second\")])), (11, Put([Variable(\"sum\")])), (12, Set(\"first\", [Variable(\"second\")])), (13, Set(\"second\", [Variable(\"sum\")])), (14, Set(\"n\", [Variable(\"n\"), Operator(Subtract), Value(Int(1))])), (15, Dctx), (15, Jmp(7)), (0, End)])",
    );
    assert_eqv(
        "\
        Never gonna let a down
        Never gonna give a 5
        Inside we both know a == 5
            Never gonna say TRUE
        Your heart's been aching but you're too shy to say it
        ",
        "Ok([(1, Let(\"a\")), (2, Set(\"a\", [Value(Int(5))])), (3, Jmpif([Variable(\"a\"), Operator(Equals), Value(Int(5))], 4)), (3, Jmp(7)), (3, Pctx), (4, Put([Value(Bool(true))])), (5, Dctx), (0, End)])",
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
