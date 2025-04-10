fn main() {

    println!("==============================================================");
    println!("==============================================================");
    println!("====                Variables in Rust                     ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                     Integers                         ====");
    println!("==============================================================");

    let default_integer = 5;

    println!("The value of default_integer (32 bits) is: {}", default_integer);

    let integer_8_bit: i8 = 1;

    println!("The value of integer_8_bit is: {}", integer_8_bit);

    let integer_16_bit: i16 = 5;

    println!("The value of integer_16_bit is: {}", integer_16_bit);

    let integer_32_bit: i32 = 10;

    println!("The value of integer_32_bit is: {}", integer_32_bit);

    let integer_64_bit: i64 = 100;

    println!("The value of integer_64_bit is: {}", integer_64_bit);

    let integer_128_bit: i128 = 1000;

    println!("The value of integer_128_bit is: {}", integer_128_bit);

    let integer_arch: isize = 10000;

    println!("The value of integer_arch is: {}", integer_arch);

    let unsigned_integer_32_bit: u32 = 1000;

    println!("The value of unsigned_integer_32_bit is: {}", unsigned_integer_32_bit);

    let unsigned_integer_64_bit: u64 = 10000;

    println!("The value of unsigned_integer_64_bit is: {}", unsigned_integer_64_bit);

    println!("==============================================================");
    println!("====                     Converting                       ====");
    println!("==============================================================");

    println!("Rust does not automatically convert types");
    println!("We can convert types using the 'as' keyword");
    println!("We can also use the 'from' method to convert types");

    let integer_32_bit: i32 = 1000;
    println!("The value of 32 bit integer is {} ... and it is of type ... {}", integer_32_bit, get_type_of(&integer_32_bit));

    let integer_64_bit = integer_32_bit as i64;
    println!("The value of integer_64_bit is: {} ... and it is of type ... {}", integer_64_bit, get_type_of(&integer_64_bit));

    println!("==============================================================");
    println!("====                     Literals                         ====");
    println!("==============================================================");

    let decimal_literal = 98_222;

    println!("The value of decimal_literal is: {}", decimal_literal);

    let hex_literal = 0xff;

    println!("The value of hex_literal is: {}", hex_literal);

    let octal_literal = 0o77;

    println!("The value of octal_literal is: {}", octal_literal);

    let binary_literal = 0b1111_0000;

    println!("The value of binary_literal is: {}", binary_literal);

    let byte_literal = b'A';

    println!("The value of byte_literal is: {}", byte_literal);

    println!("==============================================================");
    println!("====                    Constants                         ====");
    println!("==============================================================");

    const DEFAULT_CONSTANT: i32 = 5;

    println!("The value of DEFAULT_CONSTANT is: {}", DEFAULT_CONSTANT);

    const TEN_MINUTES: i32 = 10 * 60;

    println!("The value of TEN_MINUTES is: {}", TEN_MINUTES);

    println!("==============================================================");
    println!("====                     Floats                           ====");
    println!("==============================================================");

    let default_float = 5.0;

    println!("The value of default_float is: {}", default_float);

    let float_32_bit: f32 = 10.0;

    println!("The value of float_32_bit is: {}", float_32_bit);

    let float_64_bit: f64 = 100.0;

    println!("The value of float_64_bit is: {}", float_64_bit);

    println!("... to convert from an integer to a float we can use the 'as' keyword ...");

    let integer_32_bit: i32 = 1000;

    let float_32_bit = integer_32_bit as f32;

    println!("The value of float_32_bit is: {}", float_32_bit);

    println!("==============================================================");
    println!("====                     Booleans                         ====");
    println!("==============================================================");

    let default_boolean = true;

    println!("The value of default_boolean is: {}", default_boolean);

    let another_boolean: bool = false;

    println!("The value of another_boolean is: {}", another_boolean);

    println!("==============================================================");
    println!("====                     Characters                       ====");
    println!("==============================================================");
    println!("... characters are 4 bytes in size ...");
    println!("... characters can also be unicode ...");
    println!("... characters are enclosed in single quotes ...\n");
    
    let default_character = 'A';
    println!("The value of default_character is: {}", default_character);
    
    let another_character: char = 'B';
    println!("The value of another_character is: {}", another_character);

    let unicode_character = '😻';
    println!("The value of unicode_character is: {}", unicode_character);

    println!("\n... characters can also be escaped ...");
    println!("... eg newline, tab, backspace, carriage return, single quote, double quote, backslash ...");

    let newline_character = '\n';
    println!("The value of newline_character is: {}", newline_character);

    let tab_character = '\t';
    println!("The value of tab_character is: {}", tab_character);

    let backspace_character = '\x08';
    println!("The value of backspace_character is: {}", backspace_character);

    let carriage_return_character = '\r';
    println!("The value of carriage_return_character is: {}", carriage_return_character);

    let single_quote_character = '\'';
    println!("The value of single_quote_character is: {}", single_quote_character);

    let double_quote_character = '\"';
    println!("The value of double_quote_character is: {}", double_quote_character);

    let backslash_character = '\\';
    println!("The value of backslash_character is: {}", backslash_character);

    println!("\n... to convert a string to a character, we can use the 'chars' method ...");
    let string = "Hello, World!";
    let character = string.chars().next().unwrap();
    println!("\nThe value of character is: {}", character);

    println!("... to convert a character to a string, we can use the 'to_string' method ...");
    let character = 'A';
    let string = character.to_string();
    println!("\nThe value of string is: {}", string);

    println!("\n... to convert a string to an array of characters we use the 'chars' method ...");
    let string = "Hello, World!";
    let characters: Vec<char> = string.chars().collect();
    println!("The value of characters is: {:?}", characters);

    println!("\n... to convert a character to a numeric value we can use the 'to_digit' method ...");
    let character = '9';
    let number = character.to_digit(10).unwrap();
    println!("The value of number is: {}", number);

    println!("==============================================================");
    println!("====                     Tuples                           ====");
    println!("==============================================================");

    println!("Tuples are a way of grouping together a number of values with a variety of types into one compound type");

    println!("Tuples have a fixed length: once declared, they cannot grow or shrink in size");

    println!("Tuples can have elements of different types");

    println!("Tuples are useful when you want to return multiple values from a function");

    println!("Tuples are created by enclosing the values in parentheses and separating them with commas");

    println!("Tuples can be destructured to get the individual values");

    println!("Tuples can be accessed by using a period (.) followed by the index of the value");

    println!("Unlike structs, tuples do not have named fields");

    let default_tuple = (1, 2, 3, 4, 5);

    println!("The value of default_tuple is: {:?}", default_tuple);

    let another_tuple: (i32, f64, u8) = (100, 100.0, 100);

    println!("The value of another_tuple is: {:?}", another_tuple);

    println!("==============================================================");
    println!("====                     Arrays                           ====");
    println!("==============================================================");

    let default_array = [1, 2, 3, 4, 5];

    println!("The value of default_array is: {:?}", default_array);

    let another_array: [i32; 5] = [100, 200, 300, 400, 500];

    println!("The value of another_array is: {:?}", another_array);

    println!("==============================================================");
    println!("====                     Slices                           ====");
    println!("==============================================================");

    let default_slice = &default_array[1..3];

    println!("The value of default_array 1, 2, 3, 4, 5] sliced [1..3] goes from index 1 up to and less than index 3");
    println!("ie inclusive indexes 1 and 2 which in this case is numbers 2 and 3: {:?}", default_slice);

    let another_slice = &another_array[2..4];

    println!("The value of another_array [100, 200, 300, 400, 500] sliced [2..4] goes from index 2 up to and less than index 4");
    println!("ie inclusive indexes 2 and 3 which in this case is numbers 300 and 400: {:?}", another_slice);

    println!("==============================================================");
    println!("====                     Strings                          ====");
    println!("==============================================================");
    println!("Strings are UTF-8 encoded\n");

    let string_literal = "Hello, World!";
    println!("string literals are immutable ... eg {}", string_literal);

    let another_string: String = String::from("Hello, Rust!");
    println!("String::from are also immutable ... eg {}", another_string);
    
    println!("concatenate ... uses push_str method ...");
    let mut mutable_string: String = String::from("Hello,");
    mutable_string.push_str(" Mutable Rust String!");
    println!("mutable string ... push_str ... produces ... {}", mutable_string);

    println!("==============================================================");
    println!("====               Parsing String As Integer              ====");
    println!("==============================================================");

    let string = "1000";
    let integer: i32 = string.parse().unwrap();
    println!("The value of integer is: {}", integer);

    let string = "1000";
    let integer = str::parse::<i32>(string).unwrap();
    println!("The value of integer is: {}", integer);

    let string = "1000";
    let integer = i32::from_str_radix(string, 10).unwrap();
    println!("The value of integer is: {}", integer);

    let string = "1000";
    let integer = string.parse::<i32>().unwrap();
    println!("The value of integer is: {}", integer);

    println!("\n... the parse method returns a Result type which is either Ok or Err and we can unwrap an Ok result ...");
    let string = "some text string with no numeric digits";
    let integer = string.parse::<i32>().unwrap_or(0);
    println!("The value of integer is: {}", integer);

    let string = "some text string with no numeric digits";
    let parse_result = string.parse::<i32>();
    match parse_result {
        Ok(value) => println!("The value of integer is: {}", value),
        Err(_) => println!("The string could not be parsed as an integer"),
    }
    
    println!("==============================================================");
    println!("====                     Shadowing                        ====");
    println!("==============================================================");

    println!("Shadowing is when you declare a new variable with the same name as a previous variable. The new variable shadows the previous variable.");

    let shadowing_variable = 5;

    println!("The value of shadowing_variable is: {}", shadowing_variable);

    let shadowing_variable = shadowing_variable + 5;

    println!("The value of shadowing_variable is: {}", shadowing_variable);

    {
        let shadowing_variable = shadowing_variable * 2;

        println!("The value of shadowing_variable in the inner scope is: {}", shadowing_variable);
    }

    println!("The value of shadowing_variable in the outer scope is: {}", shadowing_variable);

    println!("note that we can also change the type of the variable when shadowing");

    println!("==============================================================");
    println!("====                    Blocks                            ====");
    println!("==============================================================");

    println!("Blocks are expressions in Rust");

    let block_variable = {
        let x = 1;
        let y = 2;

        println!("note that this closing statement does not have a semicolon");
        println!("this is because the block is an expression and expressions do not have semicolons");
        println!("if a semicolon is added, the block will not return a value");
        println!("this is returning a value without explicitly writing the return keyword");
        x + y
    };

    println!("The value of block_variable is: {}", block_variable);

    println!("==============================================================");
    println!("====                 Integer Overflow                     ====");
    println!("==============================================================");

    let overflow_variable: u8 = 255;

    println!("The value of overflow_variable is: {}", overflow_variable);

    // let overflow_variable = overflow_variable + 1;

    // println!("The value of overflow_variable is: {}", overflow_variable);

    println!("the above code will not compile because Rust will not allow integer overflow by default");
    println!("to allow integer overflow you can use the wrapping_add method");
    
    let overflow_variable = overflow_variable.wrapping_add(1);

    println!("The value of overflow_variable is: {}", overflow_variable);

    println!("also can consider the checked_add method, the overflowing_add and the saturating_add methods");

    println!("==============================================================");
    println!("====                        Nulls                         ====");
    println!("==============================================================");

    println!("rust does not have a null value");


    println!("==============================================================");
    println!("====             The Underscore Variable                  ====");
    println!("==============================================================");

    println!("The underscore variable is a special variable that is used to ignore a value\n");
    let _ignored_variable = 100;
    println!("The value of _ignored_variable is never used and the compiler will never complain ... ");

    println!("... we can also directly assign an underscore variable ... and it is immediately ignored ... ");
    let _ = 100;

    println!("... this can be useful in destructuring tuples ... when we only want some and not all values ... ");
    let (x, _, z) = (1, 2, 3);
    println!("The value of x is: {} and z is: {}", x, z);

    println!("\n... the underscore variable can also be used in other instances ... ");
    println!("... for example in the 'Some' variant of the Option enum ... ");

    let some_value: Option<i32> = Some(100);

    if let Some(_) = some_value {
        println!("... value is Some ...");
    } else {
        println!("... value is None ... ");
    }

    println!("\n... for example in the 'Ok' variant of the Result enum ... ");    

    let ok_value: Result<i32, i32> = Ok(100);

    if let Ok(_) = ok_value {
        println!("... value is Ok ...");
    } else {
        println!("... value is Err ... ");
    }

    println!("\n... for example in the 'Err' variant of the Result enum ... ");

    let err_value: Result<i32, i32> = Err(100);

    if let Err(_) = err_value {
        println!("... value is Err ...");
    } else {
        println!("... value is Ok ... ");
    }

    println!("\n... we can also use the underscore variable in function parameters ... ");

    fn ignore_parameter(_: i32) {
        println!("... the parameter is ignored ... ");
    }

    ignore_parameter(100);

    println!("\n... we can also use the underscore variable as a catchall in the 'match' expression ... ");

    let some_value: Option<i32> = Some(100);

    match some_value {
        Some(_) => println!("... value is Some ..."),
        None => println!("... value is None ... "),
    }

    let some_value: i32 = 3;

    match some_value {
        1 => println!("... value is 1 ..."),
        2 => println!("... value is 2 ..."),
        _ => println!("... value is something else ... "),
    }

    println!("\n... we can also use the underscore variable as a catchall in the 'if let' expression ... ");

    let some_value: Option<i32> = Some(100);

    if let Some(_) = some_value {
        println!("... value is Some ...");
    } else {
        println!("... value is None ... ");
    }


    println!("==============================================================");
    println!("====               Option Some and None                   ====");
    println!("==============================================================");

    println!("... Option Some and None are used together to define a the outcome of a function that may or may not return a value ... ");
    println!("... the desired type that we want output is defined as <T> in the Option<T> enum ... ");
    println!("... if it is present the Some(<T>) is returned ... which indicates that some value of type <T> is present ... ");
    println!("... alternatively None is returned which indicates that no value of type <T> was returned ... \n\n");

    let some_value: Option<i32> = Some(100);

    match some_value {
        Some(value) => println!("... the value is: {}", value),
        None => println!("... no value was returned ... "),
    }

    let none_value: Option<i32> = None;

    match none_value {
        Some(value) => println!("... the value is: {}", value),
        None => println!("\n... no value was returned ... "),
    }

    fn return_some_or_none(value: i32) -> Option<i32> {
        if value > 0 {
            return Some(value);
        } else {
            return None;
        }
    }

    let some_value = return_some_or_none(100);

    match some_value {
        Some(value) => println!("... the value is: {}", value),
        None => println!("... no value was returned ... "),
    }

    let none_value = return_some_or_none(-100);

    match none_value {
        Some(value) => println!("... the value is: {}", value),
        None => println!("... no value was returned ... "),
    }
    
    println!("==============================================================");
    println!("====                End of Variables in Rust              ====");
    println!("==============================================================");

    println!("==============================================================");
    println!("==============================================================");
    println!("====                  Operators In Rust                   ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                     Arithmetic                       ====");
    println!("==============================================================");

    let sum = 5 + 10;

    println!("addition - the value of 5 + 10 is: {}", sum);

    let difference = 95.5 - 4.3;

    println!("subtraction - the value of 95.5 - 4.3 is: {}", difference);

    let product = 4 * 30;

    println!("multiplication - the value of 4 * 40 is : {}", product);

    let quotient = 56.7 / 32.2;

    println!("division - the value of 56.7 / 32.2 is: {}", quotient);

    let remainder = 43 % 5;

    println!("remainder (% modulus operator) - the value of 43 % 5 is: {}", remainder);

    let truncated_division = 43 / 5;

    println!("integer division - the value of 43 / 5 is: {}", truncated_division);

    println!("==============================================================");
    println!("==============================================================");
    println!("====                 Functions In Rust                    ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                     Functions                        ====");
    println!("==============================================================");

    let default_function = default_function();

    println!("The value of default_function is: {}", default_function);

    let another_function: i32 = another_function(100);

    println!("The value of another_function is: {}", another_function);

    println!("note that statements do not return values, expressions evaluate to a resulting value");

    println!("==============================================================");
    println!("====              End of Functions In Rust                ====");
    println!("==============================================================");


    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Control Flow                      ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                     If Expressions                   ====");
    println!("==============================================================");

    let number = 3;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    let another_number = 10;

    if another_number != number {
        println!("The numbers are different");
    } else {
        println!("The numbers are the same");
    }

    let condition = true;
    let yet_another_number = if condition { 5 } else { 6 };

    println!("The value of yet_another_number is: {}", yet_another_number);

    println!("==============================================================");
    println!("====                     Loops                            ====");
    println!("==============================================================");

    let mut counter = 0;

    let result = loop {

        println!("The value of counter is: {}", counter);

        counter = counter + 1;

        if counter == 5 {
            break counter;
        }
    };

    println!("note that a loop can return a value using the break keyword");
    println!("The value of breaking out of the loop is : {}", result);

    println!("==============================================================");
    println!("====                    While Loops                       ====");
    println!("==============================================================");

    let mut counter = 0;

    while counter < 5 {
        println!("The value of counter is: {}", counter);

        counter = counter + 1;
    }

    println!("==============================================================");
    println!("====                  For Loops                           ====");
    println!("==============================================================");

    for number in (1..4).rev() {
        println!("The value of number is: {}", number);
    }

    println!("... we can make the loop inclusive by using ..= ");

    for number in (1..=4).rev() {
        println!("The value of number is: {}", number);
    }

    println!("==============================================================");
    println!("====         For Every Element In A Collection            ====");
    println!("==============================================================");

    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value of element is: {}", element);
    }

    println!("==============================================================");
    println!("====                     Arrays                           ====");
    println!("==============================================================");

    println!("Arrays are collections of elements of the same type");
    println!("Arrays have a fixed length: once declared, they cannot grow or shrink in size");
    println!("Arrays are useful when you want to store a fixed number of elements");
    println!("Arrays are created by enclosing the values in square brackets and separating them with commas");
    println!("Arrays can be accessed by using a period (.) followed by the index of the value");
    println!("... we can iterate over the array to get both the index and the value if required ... ");

    let array = [10, 20, 30, 40, 50];

    for (index, element) in array.iter().enumerate() {
        println!("The value of element at index {} is: {}", index, element);
    }

    println!("==============================================================");
    println!("====                      Loop Labels                     ====");                 
    println!("==============================================================");

    println!("Loop labels are used to break out of nested loops");

    'outer_loop: for x in 0..10 {
        for y in 0..10 {
            if x == 2 && y == 1 {
                println!("The value of x is: {} and y is: {}", x, y);
                break 'outer_loop;
            }
        }
    }
}

fn default_function() -> i32 {
    return 100;

    // or can just write 
    // 100
    // which has no 'return' keyword and no semicolon either
}

fn another_function(value: i32) -> i32 {
    return value;
}

fn get_type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}
