# Functions

## Verse Blocks and Execution

In Rickroll, ```[Verse]``` blocks act as functions, or procedures. Verse blocks can only be declared outside of the Chorus blocks.

A function declaration starts with ```[Verse NAME]```, where ```NAME``` follows the same conventions as a variable name. A name may be shared between a variable and function.

```
[Verse foo]
```

```
Syntax Error: No argument specification for function foo
```

Here, we got a syntax error since we haven't indicated what parameters foo will take. All functions must have a constant number of arguments known at runtime by name. 

We must indicate arguments on the line following the verse declaration, using the syntax ```(Ooh give you ARG1, ARG2...)```. When foo is called, the passed varlues will be initialized as variables with the names of these arguments.

If a verse takes no arguments, the keyword ```up``` can be used in place of ```ARG1, ARG2...```.

```
[Verse foo]
(Ooh give you first, second, third)
Never gonna say first
Never gonna say second
Never gonna say third

[Chorus]
```

```
```

There is no output here because although we've declared the function, we haven't actually called it anywhere from Chorus. To call a function, use a call statement (```Never gonna run FUNC and desert ARG1, ARG2, ..., ARGN```).

Note that ```ARG1, ARG2...``` must be variables and not expressions.

Also note that functions can be recursively called, but there is a 1e4 max recursion depth.

```
[Verse foo]
(Ooh give you first, second, third)
Never gonna say first
Never gonna say second
Never gonna say third

[Chorus]
Never gonna let a down
Never gonna let b down
Never gonna let c down
Never gonna give a 5
Never gonna give b -5
Never gonna give c 4
Never gonna run foo and desert a, b, c
```

```
5
-5
4
```

## Return Statements

We can also return values from functions. By default, if a function finished execution without any explicit return statements, the ```UNDEFINED``` constant is returned.

To explicitly return a value, we can use ```(Ooh give you VAR) Never gonna run FUNC and desert ARG1, ARG2...```, where ```VAR``` is an already declared variable in the current context. ```FUNC``` will be executed, and then ```VAR``` will be set to its returned value.

```
[Verse fib]
(Ooh give you a)
Inside we both know a <= 1
  (Ooh) Never gonna give, never gonna give (give you a)
Your heart's been aching but you're too shy to say it
Never gonna let b down
Never gonna let c down
Never gonna give b a - 1
Never gonna give c a - 2
(Ooh give you b) Never gonna run fib and desert b
(Ooh give you c) Never gonna run fib and desert c
(Ooh) Never gonna give, never gonna give (give you b + c)

[Chorus]
Never gonna let a down
Never gonna give a 10
(Ooh give you a) Never gonna run fib and desert a
Never gonna say a
```

```
55
```

## Intro Blocks

In addition to Verse and Chorus blocks, there is another special block called ```[Intro]```. If present, it is executed before Chorus is executed, and it executes in the global scope.

This means that all variables declared in the Intro block may be used in all functions.

```
[Intro]
Never gonna let a down
Never gonna give a 1.0

[Chorus]
Never gonna say a
```

```
1.0
```
