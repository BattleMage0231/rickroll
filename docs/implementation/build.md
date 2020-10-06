# The Build Process

Before Rickroll is executed, it must first be compiled into bytecode, which allows the interpreter to easily pattern match statements and be more organized.

## Step 1 - Lexing

When Rickroll source code files are provided to the binary, it first uses the [lexer](../../src/lexer.rs) to parse the text into an intermediate representation (IR) of your code. 

The lexer achieves this by matching pre-compiled regexes on each line. This is expensive, but requires much less code than implementing a Parse Tree.

For example, the following source may be transformed like this.

```
[Verse a]
Never gonna say 1 + 2

[Chorus]
Never gonna run a and desert you
Never gonna let x down
```

```
Verse("a", [])
Say([1, +, 2])
Chorus()
Run("a", [])
Let("x")
```

Note that the ```[]``` in ```Verse``` amd ```Run``` represent arguments to the function, which is empty in this case.

The lexer also simultaneously does syntax checking on the code, and will return a syntax error if one is present in the code.

## Step 1.5 - Tokenization

When the lexer encounters expressions in its source code (ex. "1 + 2", "TRUE || FALSE
), it uses the [tokenizer](../../src/tokenizer.rs) to parse these expressions.

## Step 2 - Optimization

After lexing, the IR will be optimized accoding to its optimization settings by the [optimizer](../../src/optimizer.rs). However, the optimizer is not yet implemented.

For now, IR is directly compiled without optimization.

## Step 3 - Compilation

Finally, the IR is compiled into Rickroll bytecode, which makes the interpreter much easier to implement. Bytecode is talked about more thoroughly in the section of the documentation dedicated towards it. 

The [compiler](../../src/compiler.rs) might transform the following IR like this.

```
Verse("fib", ["a"])
Check([a, <=, 1])
Return([a])
IFEnd()
Let("b")
Let("c")
Assign("b", [a, -, 1])
Assign("c", [a, -, 2])
RunAssign("b", "fib", ["b"])
RunAssign("c", "fib", ["c"])
Return([b, +, c])
Chorus()
Let("a")
Assign("a", [20])
RunAssign("a", "fib", ["a"])
Say([a])
```

```
function [Main]
  000   pctx
  001   let     a
  002   set     a       [Value(Int(20))]
  003   pushq   a
  004   scall   a       fib
  005   put     [Variable("a")]
  006   dctx
  007   ret     [Value(Undefined)]
function fib
  000   pctx
  001   exp     a
  002   jmpif   [Variable("a"), Operator(LessEquals), Value(Int(1))]    004
  003   jmp     007
  004   pctx
  005   ret     [Variable("a")]
  006   dctx
  007   let     b
  008   let     c
  009   set     b       [Variable("a"), Operator(Subtract), Value(Int(1))]
  010   set     c       [Variable("a"), Operator(Subtract), Value(Int(2))]
  011   pushq   b
  012   scall   b       fib
  013   pushq   c
  014   scall   c       fib
  015   ret     [Variable("b"), Operator(Add), Variable("c")]
  016   dctx
  017   ret     [Value(Undefined)]
```

## Step 4 - Interpretation

After the code is compiled, it may be interpreterd by the [interpreter](../../src/interpreter.rs). The compiler will first call the ```[Global]``` function if it exists. Then, it will call the ```[Main]``` function.

## Step 4.5 Parsing

Recall that expressions are stored in bytecode as a blueprint of the entire expression, since variables cannot be substituted at compile time.

During execution, expressions are executed in context by the [parser](../../src/parser.rs). The parser is implemented in one pass using Dijkstra's Shunting-yard algorithm.
