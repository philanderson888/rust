fn main() {
    
    println!("==============================================================");
    println!("==============================================================");
    println!("====                    File Handling                     ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                    File Creation                     ====");
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

    println!("==============================================================");
    println!("====                    File Reading                      ====");
    println!("==============================================================");

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
    println!("====                 Appending To A File                  ====");
    println!("==============================================================");

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


}
