# Language Basics

## Datatypes and Operators
Currently, rickroll supports the following data types.

| Data Type   | Stores                                                        |
|-------------|:-------------------------------------------------------------:|
| INT         | a 32-bit signed integer                                       |
| FLOAT       | an 32-bit floating point number                               |
| BOOL        | TRUE or FALSE                                                 | 
| CHAR        | a character                                                   | 
| ARRAY       | a list of data types (not yet implemented)                    | 
| UNDEFINED   | nothing (a variable's value before initialization)            | 

Operators can be used to perform operations on data types. The following operators are supported and evaluated in order.

| Operator | Action                                                      | Precedence    |
|----------|:-----------------------------------------------------------:|:-------------:|
| -        |  unary minus                                                | 0             |
| :        |  array access                                               | 1             |
| !        |  boolean not                                                | 2             |
| *        |  multiplication                                             | 3             |
| /        |  division (integer division if both arguments are integers) | 3             |
| %        |  modulo                                                     | 3             |
| +        |  addition                                                   | 4             |
| -        |  subtraction                                                | 4             |
| >        |  greater than                                               | 5             |
| <        |  less than                                                  | 5             |
| >=       |  greater than or equals                                     | 5             |
| <=       |  less than or equals                                        | 5             |
| ==       |  equals                                                     | 5             |
| !=       |  not equals                                                 | 5             |
| &&       |  boolean AND                                                | 6             |
| \|\|     |  boolean OR                                                 | 7             |

Expressions are formed by combining data types and operators. Expressions may also contain parenthesis for evaluation priority. For example, ```3 + 4 * (6 % 3) > 1``` is a valid expression. It returns ```TRUE```.

## Structure of a Program

Currently, Rickroll has a relatively simple program structure. For now, we will do everything under a ```[Chorus]``` block, which is like the main function in other languages. There can only be one Chorus block in a single program.

In the future, we will introduce ```[Verse]``` and ```[Intro]``` blocks, which will go outside the Chorus block.

```
[Chorus]
...all of our code will go here
```

## Printing to Terminal

A print statement writes to stdout the result of evaluating its argument ended by a newline. Its syntax is ```Never gonna say ARG```.

Note that any leading and trailing whitespace is automatically trimmed by the lexer.

```
[Chorus]
Never gonna say 1 + 2
Never gonna say TRUE || FALSE
```

```
3
TRUE
```

## Variables

Recall that Rickroll is a dynamically-typed language. This means that variable types are inferred at runtime and variables may be assigned a value of a different type.

To declare a variable, use the syntax ```Never gonna let VAR down```, where ```VAR``` can only have English letters and underscores.

The initial value of a variable is UNDEFINED, a special data type.

```
[Chorus]
Never gonna let a down
Never gonna let b down
Never gonna say a
```

```
UNDEFINED
```

To assign value to variables, use the syntax ```Never gonna give VAR EXPR```, where ```VAR``` is the variable name and ```EXPR``` is the expression.

```
[Chorus]
Never gonna let a down
Never gonna let b down
Never gonna give a 3 + 4
Never gonna give b a < 3
Never gonna say a
Never gonna say b
```

```
7
FALSE
```
