use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====               Grep Sample Application                ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Grep is a command-line utility for searching plain-text data sets for lines that match a regular expression");
    println!("Its name comes from the ed command g/re/p (globally search a regular expression and print)");
    println!("which has the same effect: doing a global search with the regular expression and printing all matching lines");

    println!("\n... the idea will be to combine the learning to date to build a real-world application");

    println!("\ninputs: path to file, string to search for");
    println!("\noutputs: lines containing the string");
    println!("\nconfiguration: environment variables, command-line arguments ... ");
    println!("\n... eg case sensitive/insensitive search, whole word search, line numbers, file names, etc");
    println!("\nerror handling: invalid inputs, invalid UTF-8, file not found");

    println!("\n==============================================================");
    println!("====              Accepting Command Line Arguments        ====");
    println!("==============================================================");

    println!("\n... the first step is to accept command line arguments");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    println!("args: {:?}", args);

    println!("\nunwrap or else allows us to handle errors in a more user-friendly way");
    println!("... if result is OK then unwrap ... else result is Err so handle it gently ...");
    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    println!("\n==============================================================");
    println!("====                Error Handling                        ====");
    println!("==============================================================");

    println!("... here we have a good pattern for error handling with input validation ...");
    println!("... pass input parameters to a function that returns a Result ...");
    println!("... if the Result is OK then continue ... else handle the error ...");
    println!("... we then call the 'run' function to execute the main handling of the application ...");
    println!("... the 'run' function returns a Result with a Boxed Error ...");
    println!("... this allows us to handle any errors in a generic way ...");
    println!("... with a clean and user friendly exit and user message ...");

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }    
}
