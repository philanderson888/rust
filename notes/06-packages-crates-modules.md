# packages crates modules

## contents
- [packages crates modules](#packages-crates-modules)
  - [contents](#contents)
  - [introduction](#introduction)
  - [summary](#summary)
  - [crates](#crates)
  - [package](#package)
  - [modules](#modules)
  - [submodules](#submodules)
  - [paths](#paths)
  - [worked examples](#worked-examples)
  - [crates-01](#crates-01)
  - [crates-02](#crates-02)
  - [paths](#paths-1)
  - [pub keyword](#pub-keyword)
  - [use keyword](#use-keyword)
  - [as keyword](#as-keyword)
  - [re-exporting with `pub use`](#re-exporting-with-pub-use)
  - [external packages](#external-packages)
  - [separating modules into different files](#separating-modules-into-different-files)

## introduction

we can grow our code into

- multiple modules

one package can contain

- multiple binary crates
- one library crate

as a package grows one can export parts into separate `crates` which become external dependencies

for large projects we can use `workspaces` to relate packages together

we can also encapsulate and re-use code 

as your project grows you can use scope to determine what is visible and what is not.  At any one time only one item of a given name can be in scope.

## summary

- package
  - feature of `cargo` library manager which allows you build test and share `crates`
- crates
  - tree of `modules` to produce
    - library
    - or
    - executable
- modules
  - control privacy and use of paths
- paths
  - way to name items


## crates

crate is smallest amount of code considered by the compiler at a time

crates can contain modules

crate is either

- binary crate 
  - these compile to executables which can run
  - src/main.rs is start point
  - main() starts the program
  - multiple binary crates will live in the src/bin folder
- library crate
  - src/lib.rs is start point
  - dont compile to executable
  - just contain code which is shared amongst multiple projects


in general a `crate` is a `library crate` not an executable crate

crates have a `root`

## package

a package is a bundle of `crates`

a package contains a `toml` file to define library (crate) versions

a package can contain many binary crates but only one library crate


## modules

code within a module is private by default

`use` keyword

`pub` keyword

modules are declared in the crate root file which is either `src/main.rs` or `src/lib.rs`

`mod garden` declares new module called garden

code is found in either

mod garden {}

or

src/garden.rs

or

src/garden/mod.rs

## submodules

submodules are declared as modules

`mod vegetable`

code will be 

mod vegetables {}

or 

src/garden/vegetables.rs

or

src/garden/vegetables/mod.rs

## paths

once we have defined modules then we have the path to any type in any module

eg type `asparagus` in `vegetables` in `garden` has naming convention

`crate::garden::vegetables::asparagus`

`use crate::garden::vegetables::asparagus` will permit use of the word `asparagus` only from now on to name this item, not requiring the full namespace

## worked examples

## crates-01

contains the basic structure as outlined in the docs

this all compiles and runs

## crates-02

this creates a library crate using

```bash
cargo new restaurant --lib
```

## paths

paths can be relative or absolute

- absolute path begins at the crate root

- in general use the absolute path

- relative path starts from current module and uses `self` or `super` or an identifier in the current module


paths are followed by double colons

## pub keyword

with structs the `pub` keyword makes the outer struct public but the inner fields are still private

c remains private

```rs
pub struct a {
  pub b: String,
  c: String,
}
```

if we make an enum public then all its fields are public

the default for enums is to be all public always anyway, else it is not much use


## use keyword

if two libraries have the same keyword we have to include the library name in the `use` statement

```rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

## as keyword

we can rename references with the `as` keyword

```rs
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## re-exporting with `pub use`

we can make imported libraries available in other modules when we export them using `pub use` which effectively makes our `use` statements public

```rs
pub use crate::front_of_house::hosting;
```

## external packages

add this to `Cargo.toml`

```rs
rand = "0.8.5"
```

and use with

```rs
use rand:Rng;
fn main() {
  let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

we still have to import standard library functions eg `Hashmap`

`std` is the standard library crate

```rs
use std::collections::HashMap;
```

we can combine import statements as well using

```rs
use std::cmp::Ordering;
use std::io;
// becomes
use std::{cmp::Ordering, io};
```

or

```rs
use std::io;
use std::io::Write;
// becomes
use std::io::{self, Write};
```

if we want to bring all items from a library into scope we use the `*` operator

```rs
use std::colllections::*;
```

in general it's better to explicitly call libraries into your code as this helps others find where they originated from

but `*` is useful when testing to bring all libraries into scope trivially

## separating modules into different files

we can see our `restaurant` library module is growing quite large so we can split it into `front_of_house` and `back_of_house`

we create `front_of_house.rs` and in the main `lib.rs` we retain

```rs
mod front_of_house;

pub use crate::front_of_house::hosting;
```

now if we wish to extract `hosting` module to its own file we can do that also

we place this file at the path

```rs
/front_of_house/hosting.rs
```

and in the parent `front_of_house.rs` we retain

```rs
pub mod hosting;
```

as a reference to the sub-module

`note` that the compiler will expect the modules to be named and arranged in this certain way; this will help as the project builds to ensure readability of libraries and their expected paths






