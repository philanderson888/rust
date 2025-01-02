use std::env;
use std::fs;
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

    let config = parse_config(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.file_path);

    println!("\n==============================================================");
    println!("====              Reading the File                        ====");
    println!("==============================================================");

    println!("\n... the next step is to read the file");

    let filename = "src/poem.txt";

    println!("\nfile path '{}'", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("\nfile contents:\n{}", contents);   
    
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 3 {
        let query = String::new();
        let file_path = String::new();
        return Config { query, file_path };
    }
    let query = &args[1];
    let file_path = &args[2];
    Config { query: query.to_string(), file_path: file_path.to_string() }
}
