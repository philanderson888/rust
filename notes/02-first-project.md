# first project

## contents
- [first project](#first-project)
  - [contents](#contents)
  - [introduction](#introduction)
  - [new project](#new-project)
  - [add code](#add-code)
  - [libraries](#libraries)
  - [variables](#variables)
  - [inputting data](#inputting-data)
  - [string to integer conversion](#string-to-integer-conversion)
  - [adding a random number library to the project](#adding-a-random-number-library-to-the-project)
  - [add random number code](#add-random-number-code)
  - [add comparison code to compare two numbers](#add-comparison-code-to-compare-two-numbers)
  - [guessing-game-04](#guessing-game-04)
  - [guessing-game-05](#guessing-game-05)


## introduction

just following this tutorial

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

## new project

build a new project

```bash
mkdir guessing-game
cd guessing-game
cargo new guessing-game-01
cd guessing-game-01
cargo run
# Hello World!
```

## add code

we build out `main.rs` as follows

```rs
use std::io;

fn main() {
    
    println!("Guess the number");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess+3);
}
```

## libraries

we use

```rs
use std::io;
```

to import and use a library, in this case the input/output library for entering data into a program from the user

## variables

```rs
// immutable
let a = 5;
// mutable
let mut b = 6;
```

```rs
String::new
```

this declares a new instance of a string variable

## inputting data

```rs
io::stdin().read_line(&mut guess)
```

this inputs user data and puts it in the mutable variable `guess`

the `&` means the data is referenced rather than treating it as a primitive.  In this way other parts of the program can point to and read and write to this same original data value, without creating copies of the data.  always the original data is sourced throughout the whole app, this makes for much cleaner code

by default, references are immutable so `&mut` is required to make the variable mutable

## error handling

the line

```rs
.expect("message")
```

allows error handling if the default operation returns an error

## string to integer conversion

we can convert a string to integer using

```rs
my_string.parse::<i32>().unwrap();
```

example

```rs
use std::io;

fn main() {
    
    println!("Guess the number");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess_as_integer = guess.trim().parse::<i32>().unwrap();

    println!("You guessed: {guess} and adding 2 gives {}", guess_as_integer + 2);
}
```

this will print out the chosen number plus add two onto it as well

## adding a random number library to the project

to add a library to the project we type

```bash
cargo add rand
```

then we check the `Cargo.toml` to view the output

```rs
[dependencies]
rand = "0.8.5"
```

if we then run `cargo build` we get a whole load of dependencies added to the project

```bash
cargo add rand
    Updating crates.io index
      Adding rand v0.8.5 to dependencies
             Features:
             + alloc
             + getrandom
             + libc
             + rand_chacha
             + std
             + std_rng
             - log
             - min_const_gen
             - nightly
             - packed_simd
             - serde
             - serde1
             - simd_support
             - small_rng
    Updating crates.io index
     Locking 15 packages to latest compatible versions
      Adding byteorder v1.5.0
      Adding cfg-if v1.0.0
      Adding getrandom v0.2.15
      Adding libc v0.2.169
      Adding ppv-lite86 v0.2.20
      Adding proc-macro2 v1.0.92
      Adding quote v1.0.37
      Adding rand v0.8.5
      Adding rand_chacha v0.3.1
      Adding rand_core v0.6.4
      Adding syn v2.0.90
      Adding unicode-ident v1.0.14
      Adding wasi v0.11.0+wasi-snapshot-preview1
      Adding zerocopy v0.7.35
      Adding zerocopy-derive v0.7.35


cargo build
  Downloaded byteorder v1.5.0
  Downloaded ppv-lite86 v0.2.20
  Downloaded quote v1.0.37
  Downloaded cfg-if v1.0.0
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded unicode-ident v1.0.14
  Downloaded zerocopy-derive v0.7.35
  Downloaded proc-macro2 v1.0.92
  Downloaded getrandom v0.2.15
  Downloaded rand v0.8.5
  Downloaded zerocopy v0.7.35
  Downloaded syn v2.0.90
  Downloaded libc v0.2.169
  Downloaded 14 crates (1.6 MB) in 0.75s
   Compiling proc-macro2 v1.0.92
   Compiling unicode-ident v1.0.14
   Compiling libc v0.2.169
   Compiling byteorder v1.5.0
   Compiling cfg-if v1.0.0
   Compiling getrandom v0.2.15
   Compiling rand_core v0.6.4
   Compiling quote v1.0.37
   Compiling syn v2.0.90
   Compiling zerocopy-derive v0.7.35
   Compiling zerocopy v0.7.35
   Compiling ppv-lite86 v0.2.20
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing-game-01 v0.1.0 (/Users/phil/github/rust/projects/guessing-game/guessing-game-01)
```


## add random number code

```rs
use std::io;
use rand::Rng;

fn main() {
    
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess_as_integer = guess.trim().parse::<i32>().unwrap();

    println!("You guessed: {guess} and adding 2 gives {}", guess_as_integer + 2);
}
```

## add comparison code to compare two numbers

to compare `guess` with `secret_number` we use

```rs
use std::cmp::Ordering;
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

## guessing-game-04

we can now add in 

```rs
loop {
    
}
```

so that the number gets queried over and over, and a break statement to end the game on a correct answer

## guessing-game-05

we can now add exception handling when the number asked for is not numeric

