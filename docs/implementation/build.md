# The Build Process

Before Rickroll is executed, it must first be compiled into an abstract syntax tree, which allows the interpreter to easily match statements.

## Step 1 - Lexical Analysis

When Rickroll source files are provided to the executable, it first uses the [lexer](../../src/lexer.rs) to parse the text into many tokens each representing a part of the program. The lexer achieves this by matching pre-compiled regexes on each line.

For example, the following source may be transformed like this.

```
[Verse a]
(Ooh give you up)
Never gonna say 1 + 2

[Chorus]
Never gonna run a and desert you
Never gonna let x down
```

```
Statement(1, "VERSE"), Name(1, "a"), Statement(3, "SAY"), Value(3, Int(1)), Operator(3, "+"), Value(3, Int(2)), Statement(5, "VERSE"), Name(5, "[CHORUS]"), Statement(6, "RUN"), Name(6, "a"), Statement(7, "LET"), Name(7, "x")
```

Note that the first integer in each token is the line in the source code it was taken from.

The lexer also simultaneously does simple syntax checking on the code, so it may output a syntax error.

When the lexer encounters expressions in the source code (ex. "1 + 2", "TRUE || FALSE
), it uses the [expression lexer](../../src/tokenizer.rs) located in a separate file to tokenize them.

## Step 2 - Parsing

After lexical analysis, the tokens are then parsed into an Abstract Syntax Tree by the [parser](../../src/parser.rs). This representation makes it easier for the interpreter to execute the program, as it represents everything recursively.

The top level of the finished product is a hash table relating the name of a function to its AST. 

```
{"[CHORUS]": Function(5, "[CHORUS]", [], [Run(6, "a", []), Let(7, "x")]), "a": Function(1, "a", [], [Say(3, Operation(Add, [Value(Int(2)), Value(Int(1))]))])}
```

The parser makes use of the separate [expression parser](../../src/tokenizer.rs) to parse expressions. The expression parser is single-pass and is implemented using Dijkstra's Shunting-yard algorithm.

## Step 3 - Execution

Finally, the code represented by the AST is executed by the interpreter. The nterpreter will first call the ```[Global]``` function if it exists. Then, it will call the ```[Main]``` function, which must exist.

After the code is compiled, it may be interpreterd by the [interpreter](../../src/interpreter.rs). The compiler will first call the ```[Global]``` function if it exists. Then, it will call the ```[Main]``` function.
