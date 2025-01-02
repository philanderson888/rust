use std::error::Error;
use std::{fs, env};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("please provide a search string and a file path");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("RUST_GREP_IGNORE_CASE").is_ok();
        println!("\nignore_case: {}", ignore_case);

        for (key, value) in std::env::vars() {
            if key.contains("RUST") {
                println!("{key}: {value}");
            }
        }

        Ok(Config { 
            query, 
            file_path,
            ignore_case, 
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("\n==============================================================");
    println!("====              Reading the File                        ====");
    println!("==============================================================");
    println!("... read the file from config file_path ... which is '{}'", config.file_path);
    println!("\nfile read error is returned as a boxed trait object ... Box<dyn Error>");
    let contents = fs::read_to_string(config.file_path)?;

    println!("\nfile contents:\n\n{}\n\n", contents);   
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
