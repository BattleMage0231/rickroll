# Legacy Comparison

## Compiling vs Interpreting

In the legacy implementation, source code was directly interpreted using regexes at run time. Although this may reduce the complexity of the code, it is immensly slow when dealing with control structures and functions, since every single line has to be pattern matched at every iteration.

```python
while pos < len(self.text):
    line = self.text[pos].strip()
    if not line:
        pass
    elif INTRO.match(line):
        if no_intro:
            return SyntaxError('[Intro] block must be the first block', pos + 1, self.file)
        no_intro = True
        self.intro_info = ([], pos + 1)
        # line is starting point of intro block
        cur_block = TT_INTRO
```

In comparison, the Rust implementation compiles the code into bytecode before execution. This drastically increases the speed of the program. It also allows optimizations on the source code before execution.

## Error Handling

Previously, errors were handled as classes inheriting from an Error base class.

Now, errors are a single type, while their types are variants of an ErrorType num. This is partly because Rust doesn't support inheritance, only the implementation of interfaces. More importantly, this is a better design decision since the error implementations (apart from Traceback) only had few differences.

```rust
pub struct Error {
    err: ErrorType,
    desc: String,
    line: Option<usize>,
    child: Box<Option<Error>>,
}
```

Also, many errors were recategorized or rewritten to better describe why they were raised.

## Language Differences

There are also a number of language differences that were only changed because of the ease of implementation using native data types and constructs.

### Integer Bounds

In Python, an integer has an arbitrary precision. While these exist in Rust, simply using the i32 data type was judged to be better.

However, this means that the INT type now has integer bounds rather than being unbounded.

```rust
pub enum RickrollObject {
    Int(i32), // i32
    ...
}
```

### Expressions as Arguments

Previously, the following code was syntactically correct.

```
[Verse a]
(Ooh give you x, y)

[Chorus]
Never gonna run a and desert 1 + 2, 3 + 4
```

This is a problem because "," acts as a delimiter between arguments. The lexer naively takes all instances of the character and tries to parse everything between them as an expression.

This could lead to errors, particularly when the character literal ',' is used.

This was changed so that functions could only take variables as arguments.

### Position of [Intro] and [Chorus] Blocks

The ```[Intro]``` and ```[Chorus]``` blocks are no longer forced to be at the top and bottom of the program, respectively, if they are present. 

All functions can only call earlier bindings, including themselves. This addition gives greater freedom to the programmer on function calls.
