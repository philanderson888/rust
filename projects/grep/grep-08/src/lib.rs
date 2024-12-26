use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("please provide a search string and a file path");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("\n==============================================================");
    println!("====              Reading the File                        ====");
    println!("==============================================================");
    println!("... read the file from config file_path ... which is '{}'", config.file_path);
    println!("\nfile read error is returned as a boxed trait object ... Box<dyn Error>");
    let contents = fs::read_to_string(config.file_path)?;
    println!("\nfile contents:\n\n{}", contents);   
    Ok(())
}
