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

    let default_character = 'A';

    println!("The value of default_character is: {}", default_character);

    let another_character: char = 'B';

    println!("The value of another_character is: {}", another_character);

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

    let string_literal = "Hello, World!";

    println!("string literal (cannot mutate) - value is : {}", string_literal);

    let another_string: String = String::from("Hello, Rust!");
    println!("string - value is : {}", another_string);

    let mut mutatable_string: String = String::from("Hello,");
    mutatable_string.push_str(" Mutable Rust String!");
    println!("mutatable string - value is : {}", mutatable_string);

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

        // note that this closing statement does not have a semicolon
        // if a semicolon is added, the block will not return a value
        // this is returning a value without explicitly writing the return keyword
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

    println!("==============================================================");
    println!("====         For Every Element In A Collection            ====");
    println!("==============================================================");

    let array = [10, 20, 30, 40, 50];

    for element in array.iter() {
        println!("The value of element is: {}", element);
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
