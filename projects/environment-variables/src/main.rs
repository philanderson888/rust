use std::env;

fn main() {

    println!("==============================================================");
    println!("====                Environment Variables                 ====");
    println!("==============================================================");
    
    println!("... print all environment variables ...");

    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }

    println!("... set an environment variable ...");

    let key = "RUST_BACKTRACE";
    unsafe {
        env::set_var(key, "1");
    }
    assert_eq!(env::var(key), Ok("1".to_string()));

    println!("... can check this variable is set ... ");

    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }

    println!("... to print just matching environment variables ...");

    for (key, value) in std::env::vars() {
        if key.contains("RUST") || key.contains("CARGO") {
            println!("{key}: {value}");
        }
    }

}
