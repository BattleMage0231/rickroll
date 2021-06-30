# Built-In Functions

## Name Shadowing

Rickroll has a number of built-in functions to allow for operations not otherwise supported by the language. These functions are written in Rust but can be called from Rickroll.

Built-in functions are special in the sense that if a user-defined function with the same name exists, the built-in function will be shadowed.

## ArrayOf

The ArrayOf function allows you to create a dynamically-sized array from a number of elements (`ArrayOf [elements...]`). The function can be called with any number of arguments and the resulting array is returned.

```
[Chorus]
Never gonna let a down
Never gonna give a 5
Never gonna let b down
Never gonna give b 10.0
Never gonna let c down
Never gonna let d down
(Ooh give you d) Never gonna run ArrayOf and desert a, b, c
Never gonna say d
```

```
[5, 10, UNDEFINED]
```

## ArrayPop

The ArrayPop function returns a new array with an element at a specified index removed (`ArrayPop [array] [index]`). It will throw an error if the index if out of bounds.

```
[Chorus]
Never gonna let a down
Never gonna give a 5
Never gonna let b down
Never gonna give b 10.0
Never gonna let c down
Never gonna let d down
(Ooh give you d) Never gonna run ArrayOf and desert a, b, c
Never gonna let e down
Never gonna give e 0
(Ooh give you d) Never gonna run ArrayPop and desert d, e
Never gonna say d
```

```
[10, UNDEFINED]
```

## ArrayPush

The ArrayPush function returns a new array with an addition element inserted at a specified index (`ArrayPush [array] [index] [value]`). It will throw an error if the index is out of bounds (strictly greater than length).

```
[Chorus]
Never gonna let a down
Never gonna give a 5
Never gonna let b down
Never gonna give b 10.0
Never gonna let c down
Never gonna let d down
(Ooh give you d) Never gonna run ArrayOf and desert a, b, c
Never gonna let e down
Never gonna give e 0
(Ooh give you d) Never gonna run ArrayPush and desert d, e, a
Never gonna say d
```

```
[5, 5, 10, UNDEFINED]
```

## ArrayReplace

The ArrayReplace function returns a new array with one element at a specified index replaced with another (`ArrayReplace [array] [index] [value]`). It will throw an error if the index is out of bounds.

```
[Chorus]
Never gonna let a down
Never gonna give a 5
Never gonna let b down
Never gonna give b 10.0
Never gonna let c down
Never gonna let d down
(Ooh give you d) Never gonna run ArrayOf and desert a, b, c
Never gonna let e down
Never gonna give e 1
(Ooh give you d) Never gonna run ArrayReplace and desert d, e, a
Never gonna say d
```

```
[5, 5, UNDEFINED]
```

## ArrayLength

The ArrayLength function calculates and returns an INT representing the length of an array (`ArrayLength [array]`).

```
[Chorus]
Never gonna let a down
Never gonna give a 5
Never gonna let b down
Never gonna give b 10.0
Never gonna let c down
Never gonna let d down
(Ooh give you d) Never gonna run ArrayOf and desert a, b, c
(Ooh give you d) Never gonna run ArrayLength and desert d
Never gonna say d
```

```
3
```

## PutChar

The PutChar function writes one character to the standard output (`PutChar [char]`). It does not append a newline character.

```
[Chorus]
Never gonna let a down
Never gonna give a 'L'
Never gonna run PutChar and desert a
```

```
L
```

## ReadLine

The ReadLine function reads a line from the standard input and returns an array of characters (`ReadLine`). It does not include newline characters.

```
[Chorus]
Never gonna let a down
(Ooh give you a) Never gonna run ReadLine and desert you
Never gonna say a
```

```
Hello World! // assuming you entered "Hello World!"
```
