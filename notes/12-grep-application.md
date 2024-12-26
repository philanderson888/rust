# grep application

## contents

- [grep application](#grep-application)
  - [contents](#contents)
  - [code](#code)
  - [versions](#versions)
    - [04](#04)
    - [05](#05)
    - [06](#06)
  - [07](#07)
  - [08](#08)
  - [09](#09)
  - [10](#10)
  - [11](#11)
  - [12](#12)

## code

code will be found in 

[grep](../projects/grep/)

## versions

### 04

update handling of the config

### 05

return a struct with the config contained in it

actually return a `Result` type with Ok and the config, or Err and handle lack of parameters gently and let the user know to run the app again with correct command line arguments

### 06

add `run(config)` function to help clean up `main()` and eventually move code to library, out of `main`

## 07

tidy up the whole run() function so it returns an object which is either OK or a boxed result which can be handled and the application terminated with a clean error message to the user

## 08

move code to the library `lib.rs` rather than the `main.rs`

## 09 

adding tests

## 10 

add case insensitive search

## 11

add case insensitive search to config struct

## 12

print std error output to screen using `eprint`

