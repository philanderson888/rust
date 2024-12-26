# ownership

## contents

- [ownership](#ownership)
  - [contents](#contents)
  - [introduction](#introduction)
  - [multiple variables](#multiple-variables)
  - [clone](#clone)

## introduction

ownership governs memory usage

ownership makes rust unique!  

ownership rules

- every value has an `owner`
- only one `owner`
- when `owner` goes out of scope the value is dropped and the `drop` function is called by rust (same as c++ raii)

see [owership](../projects/ownership/) for code matching these notes

## multiple variables

```rs
let x = 5; 
let y = x;
```

these are primitives so are created on the stack in two different memory slots so are independent

```rs
let string01 = String::from("hello");
let string02 = string01;
```

`string01` variable has 3 parts

- pointer ... address of the 0th index of the string
- len ... length of the string in bytes
- capacity ... allocated capacity ... for now ignore this and assume it's same as length

in memory the string pointer at address 0 has value `h`, 1 has `e` and so on until index 4 holds last letter `o`

`string02` just copies the pointer details exactly the same as `string01`

to prevent any errors after this point we `de-refer` `string01` when `string02` is set to equal it

ie from this point

`string01` is no longer in scope and cannot be used
`string02` is the only variable in scope

so effectively in `rust` terminology we have performed a `move` of variable from `string01` to `string02` ie renamed it!

it's just like a rename!  simple!

this also happens with mutable variables if we reassign - the original data is removed from memory, and a new variable is created

```rs
let mut my_string = String::from("hello");
my_string = String::from("goodbye");
```

the original `my_string` is completely deleted from memory and a completely new variable is created

## clone

to clone a heap object we just use

```rs
let string03 = String::from("hello");
let string04 = string03.clone();
```

note that `primitives` and `tuples` containing `primitives` create clones by default without the use of the clone keyword


