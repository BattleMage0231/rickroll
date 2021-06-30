<!-- PROJECT LOGO -->
<br />
<p align="center">
  <h2 align="center">Rickroll</h3>

  <p align="center">
    Esoteric programming language using lyrics from Never Gonna Give You Up
    <br />
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgements">Acknowledgements</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

![](https://www.aberdeen.com/wp-content/uploads/2014/07/rickrolled.jpg)

Rickroll is an esoteric programming languages whose commands are adapted from the lyrics of Never Gonna Give You Up by Rick Astley. It is a dynamically typed imperative programming language that supports variables, condition blocks (loops and if statements), functions, and block scopes. The stages of the interpreter include lexical analysis, parsing (into an AST), and execution. This language can be proven to be Turing complete because it can emulate a Turing machine.


This project was made as a learning experiment for programming languages and interpreters for scripting languages. As such, methods used in the code may not be up to industry standard and the program may be considered unsafe (especially compared to other Rust applications and libraries).

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

You must have Rust installed to built this project (tested with rustc 1.48.0). The project is packaged using Cargo, Rust's package manager. 

### Installation

1. Clone the repo and cd into it
   ```sh
   git clone https://github.com/BattleMage0231/rickroll.git
   cd rickroll
   ```
2. To build the executable or run the program, do one of the following
   ```sh
   cargo build # generates debug executable at target/debug/rickroll
   cargo build --release # generate optimized executable at target/rls/rickroll
   cargo run # build debug executable and run immediately
   ```

<!-- USAGE EXAMPLES -->
## Usage

The full language documentation and interpreter guide can be found in the [docs](./docs) directory. Following are some example code snippets.

### Recursive Fibonacci

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

### Reading User Input

```
[Chorus]
Never gonna let line down
(Ooh give you line) Never gonna run ReadLine and desert you
Never gonna say line
```

### Hello World

```
[Chorus]
Never gonna let letter_H down
Never gonna let letter_e down
Never gonna let letter_l down
Never gonna let letter_o down
Never gonna let comma down
Never gonna let space down
Never gonna let letter_W down
Never gonna let letter_r down
Never gonna let letter_d down
Never gonna let exclamation down
Never gonna let newline down
Never gonna give letter_H 'H'
Never gonna give letter_e 'e'
Never gonna give letter_l 'l'
Never gonna give letter_o 'o'
Never gonna give comma ','
Never gonna give space ' '
Never gonna give letter_W 'W'
Never gonna give letter_r 'r'
Never gonna give letter_d 'd'
Never gonna give exclamation '!'
Never gonna give newline '\n'
Never gonna let string down
(Ooh give you string) Never gonna run ArrayOf and desert letter_H, letter_e, letter_l, letter_l, letter_o, comma, space, letter_W, letter_o, letter_r, letter_l, letter_d, exclamation, newline
Never gonna let length down
(Ooh give you length) Never gonna run ArrayLength and desert string
Never gonna let i down
Never gonna give i 0
Inside we both know i < length
Never gonna let chr down
Never gonna give chr string : i
Never gonna run PutChar and desert chr
Never gonna give i i + 1
We know the game and we're gonna play it
```

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE` for more information.



<!-- CONTACT -->
## Contact

Leyang Zou - leyang.zou@student.tdsb.on.ca

Project Link: [https://github.com/BattleMage0231/rickroll](https://github.com/BattleMage0231/rickroll)



<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [GitHub README template](https://github.com/othneildrew/Best-README-Template)
