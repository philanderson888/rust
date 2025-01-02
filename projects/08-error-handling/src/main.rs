use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, ErrorKind};

fn main() {

    println!("==============================================================");
    println!("==============================================================");
    println!("====                Error Handling                        ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                  Introduction                        ====");
    println!("==============================================================");
    
    println!("Error handling is the process of handling errors that might occur in a program.");
    println!("Error handling is important because it can help prevent a program from crashing when an error occurs.");

    println!("... in rust errors are classified as recoverable and unrecoverable errors.");

    println!("... a recoverable error returns the type Result<T, E>.");
    println!("... a unrecoverable error returns the type panic!");

    println!("In Rust, error handling is done using the Result enum and the Option enum.");
    println!("The Result enum is used to handle errors that might occur in a program");
    println!("... while the Option enum is used to handle optional values.");
    println!("The Result enum has two variants: Ok and Err.");
    println!("The Ok variant is used to represent a successful result, while the Err variant is used to represent an error.");

    println!("==============================================================");
    println!("====                    Panic Errors                      ====");
    println!("==============================================================");

    println!("... when a panic error occurs, the stack is cleaned up unless the flag `panic=abort` is set.");

    println!("... we can manually call the `panic!` macro to generate a panic error.");

    use std::env;

    let key = "RUST_BACKTRACE";
    //env::set_var(key, "1");
    env::set_var(key, "0");
  
    // assert_eq!(env::var(key), Ok("1".to_string()));
    assert_eq!(env::var(key), Ok("0".to_string()));

    //env::set_var(key, "full");
    //assert_eq!(env::var(key), Ok("full".to_string()));

    println!("... before we call our error we can check our environment variable is set to `1` or `full`");

    for (key, value) in std::env::vars() {
        if key.contains("RUST") {
            println!("{key}: {value}");
        }
    }

    println!("\n... now we can call the panic error ...\n");

    //panic!("This is a panic error!");

    println!("... to get a full backtrace of all functions called up to this point ...");
    println!("... we can set the environment variable RUST_BACKTRACE=1 or RUST_BACKTRACE=full");


    println!("==============================================================");
    println!("====                    Result Errors                      ====");
    println!("==============================================================");

    println!("... the Result enum is used to handle errors that might occur in a program.");

    println!("... the Result enum has two variants: Ok and Err.");

    println!("... the Ok variant is used to represent a successful result, while the Err variant is used to represent an error.");

    println!("... the Result enum is defined as follows:");
    println!("enum Result <T E> ");
    println!("    Ok(T),");
    println!("    Err(E),");
    
    println!("... T is the type returned when there is no error");
    println!("... E is the type returned when there is an error");

    println!("==============================================================");
    println!("====                    Simple Example                    ====");
    println!("==============================================================");

    println!("... in this example we will open a file and read its contents.");
    println!("... T is std::fs::File type");
    println!("... E is std::io::Error type");

    let _file01 = File::create("Hello.txt");
    println!("... now we match the file type returned in order to know what happened");

    let _file01 = match _file01 {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem creating the file: {:?}", error)
        },
    };

    println!("... this line returns file01 of either type File or type Error depending on the result");
    let _file01 = File::open("Hello.txt");

    println!("... now we match the file type returned in order to know what happened");
    let _file01 = match _file01 {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    println!("==============================================================");
    println!("====             Slightly More Complex Example            ====");
    println!("==============================================================");

    println!("... we can now take action to create a new file when one does not exist");

    let _file02 = File::open("Hello.txt");

    let _file02 = match _file02 {
        Ok(file) => file,
        Err(_) => {
            match File::create("Hello.txt") {
                Ok(fc) => {
                    println!("... we have created a new file: Hello.txt");
                    fc
                },
                Err(e) => {
                    panic!("Tried to create and open the file but there was a problem: {:?}", e)
                },
            }
        },
    };

    println!("... in this case we explicitly ignore the error type by using the `_` placeholder");
    println!("... so we are assuming the error is to do with file creation ... which it might not be ...");

    println!("... we can create a more solid example where we exaimine the error type ...");

    let _file03 = File::open("Hello.txt");

    let _file03 = match _file03 {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                match File::create("Hello.txt") {
                    Ok(fc) => {
                        println!("... we have created a new file: Hello.txt");
                        fc
                    },
                    Err(e) => {
                        panic!("Tried to create and open the file but there was a problem: {:?}", e)
                    },
                }
            } else {
                panic!("There was a problem opening the file: {:?}", error)
            }
        },
    };

    println!("... so in this instance we have provided much more robust error handling ...");

    println!("... we can shorten this code by using the `unwrap_or_else` method ...");

    let _file04 = File::open("Hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create and open the file but there was a problem: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        }
    });

    println!("... this code is much more concise and does the same thing ...");

    println!("==============================================================");
    println!("====                        Unwrap                        ====");
    println!("==============================================================");

    println!("... the `unwrap` method is a shortcut method that returns the value of the `Ok` variant if the result is `Ok`");
    println!("... and panics if the result is `Err`");

    let _file05 = File::open("Hello.txt").unwrap();

    println!("... this code will panic if the file does not exist ...");

    println!("==============================================================");
    println!("====                      Expect                          ====");
    println!("==============================================================");

    println!("... the `expect` method is similar to the `unwrap` method, but it allows us to specify a custom error message.");

    let _file06 = File::open("Hello.txt").expect("Failed to open Hello.txt");

    println!("... this code will panic if the file does not exist ...");

    println!("==============================================================");
    println!("====                      Propagating Errors              ====");
    println!("==============================================================");

    println!("... we can propagate errors by using the `?` operator.");

    println!("... the `?` operator is used to return the value of the `Ok` variant if the result is `Ok`");
    println!("... and to return the error if the result is `Err`");

    println!("... the `?` operator can only be used in functions that return the `Result` type.");

    println!("... in this example we will read the contents of a file and return the contents as a string ...");

    let mut _file07 = OpenOptions::new()
        .append(true)
        .open("Hello.txt")
        .expect("cannot open file");

    _file07
        .write("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");

    let mut _file08 = File::open("Hello.txt");
    let mut contents = String::new();
    let _ = _file08.unwrap().read_to_string(&mut contents);

    println!("File contents: {}", contents);

    println!("... we can now refactor this code to use the `?` operator ...");

    let contents = read_username_from_file();

    match contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    let contents = read_username_from_file_short();
    
    match contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }
    
    let contents = read_username_from_file_shorter();

    match contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    let contents = read_username_from_file_shortest();

    match contents {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {:?}", e),
    }

    println!("... this code is much more concise and does the same thing ...");
    println!("... note that the ? operator can only be used in functions that return the Result type ...");

    let this_character = last_char_of_first_line("Hello World");

    match this_character {
        Some(c) => println!("Last character of first line: {}", c),
        None => println!("No last character found"),
    }

    println!("==============================================================");
    println!("====                   Input Validation                   ====");
    println!("==============================================================");

    println!("... we can use error handling, the Result<T, E> and the `?` operator to validate input ...");

    println!("... in this example we will validate a user's age ...");

    let age = "20";

    let age = match age.parse::<u32>() {
        Ok(age) => age,
        Err(_) => {
            println!("Invalid age");
            0
        },
    };

    println!("Age: {}", age);


    println!("... or we could ensure the guess of a user is within a certain range ...");

    let guess = "20";

    let guess = match guess.parse::<u32>() {
        Ok(guess) => guess,
        Err(_) => {
            println!("Invalid guess");
            0
        },
    };

    println!("Guess: {}", guess);

    println!("... and we can create a struct to hold the guess and perform the same validation ...");
    println!("... using the struct with the validation inside the constructor ...");
    println!("... this means it is impossible to create an invalid guess ...");
    println!("... and can be a very clean way to handle input validation ...");

    let guess = Guess::new(20);

    println!("Guess: {}", guess.value);

    println!("==============================================================");
    println!("====                      Summary                         ====");
    println!("==============================================================");

    println!("... in this tutorial we have learned about error handling in Rust.");
    println!("... we have learned about recoverable and unrecoverable errors.");
    println!("... we have learned about the Result and Option enums.");
    println!("... we have learned about the panic! macro.");
    println!("... we have learned about the unwrap and expect methods.");
    println!("... we have learned about propagating errors using the ? operator.");
    println!("... we have learned about input validation.");
    println!("... we have learned about the ? operator.");
    println!("... we have learned about the Result<T, E> type.");
    println!("... we have learned about the Option<T> type.");
    println!("... we have learned about the panic! macro.");
    println!("... we have learned about the unwrap and expect methods.");
    println!("... we have learned about propagating errors using the ? operator.");
    println!("... we have learned about input validation.");
    
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {
            value
        }
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("Hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("Hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("Hello2.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("Hello.txt")
}
