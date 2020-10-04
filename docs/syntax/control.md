# Loops and Control Statements

- If Statements
- While Loops
- Context and Scopes

## If Statements

An if statement has two parts. The first part (```Inside we both know EXPR```) acts like a logic gate, checking the value of its BOOL argument ```EXPR```. If the value is TRUE, the block inside the if statement is executed.

If the value if FALSE, control will skip to the end of the if statement, indicated by the second part (```Your heart's been aching but you're too shy to say it```).

```
[Chorus]
Never gonna let a down
Never gonna give a TRUE
Inside we both know a
    Never gonna say 2 + 2
Your heart's been aching but you're too shy to say it
```

```
4
```

## While Loops

A while loop is similar to an if statement, except that it will continuously execute its code until its expression evaluates to FALSE.

The syntax of a while loop is also similar to that of an if statement. The first part that checks the value of its boolean expression is exactly the same.

However, the second part indicating when the loop stops is replaced with ```We know the game and we're gonna play it```. Essentially, this tells the interpreter to jump back to the last check statement.

```
[Chorus]
Never gonna let a down
Never gonna let b down
Never gonna give a 0
Never gonna give b 5
Inside we both know a < b
  Inside we both know a % 2 == 0
    Never gonna say -a
  Your heart's been aching but you're too shy to say it
  Never gonna give a a + 1
We know the game and we're gonna play it
```

```
0
-2
-4
```

## Context and Scopes

You might have noticed that variables declared inside the code block of an if statement or while loop cannot be used outside of that block.

```
[Chorus]
Inside we both know TRUE
  Never gonna let a down
  Never gonna give a 5
Your heart's been aching but you're too shy to say it
Never gonna say a
```

```
Name Error: Variable a not found
```

This is because each of these blocks has its own context, of variables that can be used inside the block. In any block, only variables that are in scope can be used. All variables in a block go out of scope as soon as the terminating statement of that block is reached (if or while end).

```
[Chorus]
Inside we both know TRUE
    Never gonna let a down
    Inside we both know TRUE
        Never gonna say a
        Never gonna give a 4
    Your heart's been aching but you're too shy to say it
    Never gonna say a
Your heart's been aching but you're too shy to say it
Never gonna say a
```

```
UNDEFINED
4
Name Error: Variable a not found
```
