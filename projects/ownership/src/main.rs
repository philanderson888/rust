fn main() {
    
    println!("==============================================================");
    println!("==============================================================");
    println!("====                    Ownership                         ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                      Rules                           ====");
    println!("==============================================================");
    
    println!("1. Each value in Rust has a variable that’s called its owner.");
    println!("2. There can only be one owner at a time.");
    println!("3. When the owner goes out of scope, the value will be dropped.");

    println!("==============================================================");
    println!("====                      Example                        ====");
    println!("==============================================================");

    let string01 = String::from("this is a string which is moved from string01 to string02");
    let string02 = string01;

    println!("after assigning string01 to string02 then string01 goes out of scope");
    // println!("{}", string01); // This will not work
    println!("{}", string02); // This will work

    println!("==============================================================");
    println!("====                      Clone                          ====");
    println!("==============================================================");

    let string03 = String::from("this is a string which is cloned");
    let string04 = string03.clone();

    println!("after cloning string03 to string04 then both are valid");

    println!("{}", string03); // This will work
    println!("{}", string04); // This will work

    println!("==============================================================");
    println!("====        Functions - Primities Stay In Scope           ====");
    println!("==============================================================");

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    println!("{}", x);              // This will work

    println!("==============================================================");
    println!("====  Functions - Strings Go Out Of Scope When Passed In  ====");
    println!("==============================================================");

    let string05 = String::from("this string is passed into a function and goes out of scope there");  // string05 comes into scope
    takes_ownership(string05);             // string05 value moves into the function...
                                           // ... and so is no longer valid here
    //  println!("{}", string05); // This will not work

    println!("==============================================================");
    println!("====          Functions - Return A Fresh Variable         ====");
    println!("==============================================================");

    let string06 = String::from("this string is returned from a function");
    let string07 = gives_ownership(string06);         // gives_ownership moves its return
                                                      // value into string07
    println!("{}", string07);


    println!("==============================================================");
    println!("====          Functions - Passing A Reference             ====");
    println!("==============================================================");

    println!("Passing a reference to a function does not take ownership of the variable\n");
    println!("& is just a pointer to the string variable, and this is passed into the function\n");
    println!("... also references are immutable by default\n");

    let string08 = String::from("this string is passed by reference to a function");
    let string08_length = calculate_length(&string08);

    println!("The string '{}' of length {}", string08, string08_length);
    println!("   ... is still valid after the function call");

    println!("==============================================================");
    println!("====          Functions - Mutable References              ====");
    println!("==============================================================");

    println!("Passing a mutable reference to a function allows the function to modify the variable\n");
    println!("&mut is just a pointer to the string variable, and this is passed into the function\n");
    println!("... note that only one mutable reference is allowed at a time\n");
    println!("... this avoids what is called a 'data race condition' where two or more pointers access the same data, and nothing monitors the synchronisation\n");

    let mut string09 = String::from("this string is passed by mutable reference to a function");
    change_this_string(&mut string09);

    println!("The string '{}' ... is modified by the function ... and is still available after the function call as well", string09);

    println!("==============================================================");
    println!("====       Combining Mutable And Immutable References     ====");
    println!("==============================================================");

    println!("... note that you can have multiple immutable references, but only one mutable reference\n");

    let mut string10 = String::from("this string is passed by mutable reference to a function");
    let string10_length = calculate_length(&string10);
    change_this_string(&mut string10);

    println!("The string '{}' of length {} ... is modified by the function ... and is still available after the function call as well", string10, string10_length);

    println!("... however if we make multiple references it fails\n");

    let mut string11 = String::from("this string is passed by mutable reference to a function");
    change_this_string(&mut string11);
    // change_this_string(&mut string11); // This will not work

    println!("==============================================================");
    println!("====                Dangling References                   ====");
    println!("==============================================================");

    println!("... note that dangling references are not allowed in Rust\n");

    // let reference_to_nothing = dangle(); // This will not work

    println!("==============================================================");
    println!("====                 String Slices                        ====");
    println!("==============================================================");

    println!("slices are a reference to a sequence of elements in a collection\n");
    println!("... and as such have no owner");

    println!("imagine we had to iterate over a string and return the index of the first space character\n");

    let string12 = String::from("this is a string with spaces");
    let first_space = find_first_space(&string12);

    let string13 = &string12[0..first_space];
    let string14 = &string12[first_space+1..string12.len()];

    println!("The first part of the string is '{}'", string13);
    println!("The second part of the string is '{}'", string14);

    println!("... note that slices are immutable by default\n");

    println!("... note that we can also abbreviate the way we call the slices if they contain the first or last element\n");

    let string15 = String::from("this is a string with spaces");
    let first_space = find_first_space(&string15);

    let string16 = &string15[..first_space];
    let string17 = &string15[first_space+1..];

    println!("The first part of the string is '{}'", string16);
    println!("The second part of the string is '{}'", string17);

    println!("... also we can slice the whole string by using the full range operator\n");

    let string18 = String::from("this is a string with spaces");
    let string19 = &string18[..];

    println!("The whole string is '{}'", string19);

    println!("... we can now rewrite the find_first_space function to return a slice\n");

    let string20 = String::from("this is a string with spaces");

    let string21 = find_first_slice(&string20);

    println!("The first part of the string is '{}'", string21);

    println!("... note that string literals are also slices\n");

    println!("... note that `str` is a string slice type\n and as such is different to the `String` type\n");

    println!("... ... so we can define our functions to take `str` string slice types instead of `String` types\n");

    println!("... ... this is because `str` is a fixed size and is stored on the stack\n");
    println!("... ... and `String` is a growable type and is stored on the heap\n");
    println!("... ... and `String` is a reference to a sequence of bytes\n");
    println!("... ... and `str` is a reference to a sequence of bytes\n");    

    println!("... ... so we can define our functions to take `str` string slice types instead of `String` types\n");

    let my_string_literal = "this is a string literal";

    let string22 = find_first_slice_from_slice(my_string_literal);

    println!("The first part of the string is '{}'", string22);

    println!("... note that we can pass in the string literal parameter directly without explicityly adding the reference symbol\n");

    let string23 = find_first_slice_from_slice("this is a string literal");

    println!("The first part of the string is '{}'", string23);

    println!("==============================================================");
    println!("====               Non-String Slices                      ====");
    println!("==============================================================");

    println!("... note that slices can be used on any type of collection\n");

    let array01 = [1, 2, 3, 4, 5];

    let array_slice01 = &array01[1..3];

    println!("The array slice is {:?}", array_slice01);

    assert_eq!([2, 3], array_slice01);

    println!("... the array slice points to the first element and provides the length of the slice, that is all that is required\n");

    

}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership(some_string: String) -> String {     // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn calculate_length(string_in_function: &String) -> usize { 
    println!("... note that the function parameter also has to denote & as the string type to indicate that it is a reference");
    string_in_function.len()
} 

fn change_this_string(string_in_function: &mut String) {
    println!("... note that the function parameter also has to denote &mut as the string type to indicate that it is a mutable reference");
    string_in_function.push_str(" ... and this string is modified by the function");
    println!("{}", string_in_function);
}

// fn dangle() -> &String { // dangle returns a reference to a String

fn find_first_space(find_first_space_string: &String) -> usize {
    let string_as_bytes = find_first_space_string.as_bytes();

    let mut first_space_index = find_first_space_string.len();

    for (i, &item) in string_as_bytes.iter().enumerate() {
        if item == b' ' {
            println!("The first space character is at index {}", i);
            first_space_index = i;
            return first_space_index;
        }
    }

    println!("No space character found");
    return first_space_index;
    
}

fn find_first_slice(long_string: &String) -> &str {
    let string_as_bytes = long_string.as_bytes();
    for (i, &item) in string_as_bytes.iter().enumerate() {
        if item == b' ' {
            println!("The first space character is at index {}", i);
            return &long_string[..i];
        }
    }
    println!("No space character found");
    return &long_string[..];
}

fn find_first_slice_from_slice(long_slice: &str) -> &str {
    let slice_as_bytes = long_slice.as_bytes();
    for (i, &item) in slice_as_bytes.iter().enumerate() {
        if item == b' ' {
            println!("The first space character is at index {}", i);
            return &long_slice[..i];
        }
    }
    println!("No space character found");
    return &long_slice[..];
}