use std::collections::HashMap;

fn main() {

    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Collections                      ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("collections can be summarised as: ");
    println!("1. Vectors: store a variable number of values next to each other");
    println!("2. Strings: store a collection of characters");
    println!("3. Hashmaps: store a collection of key-value pairs");

    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Vectors                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Vectors are similar to arrays but can grow or shrink in size.");
    println!("They are stored on the heap rather than the stack.");
    println!("They are useful when you have a list of items that you want to store in a single variable.");
    println!("You can create a new, empty vector with the Vec::new function");
    println!("you can add elements to it with the push method");
    println!("... the capacity of a vector is the amount of space allocated for any future elements that will be added to it ...");
    println!("... if a vector's length exceeds its capacity, its elements will be reallocated to a new space ...");
    println!("... this is done to ensure that all elements are stored next to each other in memory ie are contiguous ...");
    println!("... the capacity is doubled each time a vector runs out of space ...");
    println!("You can also create a vector that has the same value for each element with the vec! macro.\n");
    println!("When a collection goes out of scope rust drops the collection and all of the individual elements\n");

    println!("==============================================================");
    println!("====                 Simple Examples                      ====");
    println!("==============================================================");

    let vector01: Vec<i32> = Vec::new();

    let vector02 = vec![1, 2, 3];

    let mut vector03 = Vec::new();
    vector03.push(5);
    vector03.push(6);
    vector03.push(7);
    vector03.push(8);

    println!("vector01: {:?}", vector01);
    println!("vector02: {:?}", vector02);
    println!("vector03: {:?}", vector03);

    println!("... to access an item by index we use ...");
    let third: &i32 = &vector03[2];
    println!("The third element of vector03 is: {}", third);
    println!("... note that if we try to access an index that is out of bounds the program will panic ...\n");

    println!("... if we want to use `Option` syntax to be sure we are reading a valid index ...");
    let fourth: Option<&i32> = vector03.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element of vector03 is: {}", fourth),
        None => println!("There is no fourth element in vector03"),
    }
    let fifth: Option<&i32> = vector03.get(4);
    match fifth {
        Some(fifth) => println!("The fifth element of vector03 is: {}", fifth),
        None => println!("There is no fifth element in vector03"),
    }

    println!("\nalso be aware if a reference to to an element is held, the vector cannot be mutated");
    println!("... the picture of this is that we should not add rungs to a ladder while standing on it ...\n");

    println!("==============================================================");
    println!("====                    Iterating                         ====");
    println!("==============================================================");

    println!("... we can iterate over the elements of a vector using a for loop ...");

    let vector04 = vec![100, 32, 57];
    for i in &vector04 {
        println!("{}", i);
    }

    println!("... we can also iterate over mutable references to each element in a mutable vector ...");

    let mut vector05 = vec![100, 32, 57];
    for i in &mut vector05 {
        *i += 50;
    }

    for i in &vector05 {
        println!("{}", i);
    }

    println!("... i is a mutable reference to each element in the vector ...");
    println!("... we use the dereference operator (*) to get the value in i and modify it ...");

    println!("==============================================================");
    println!("====                   Arrays To Vectors                  ====");
    println!("==============================================================");

    println!("... we can convert an array to a vector using the `to_vec` method ...");

    let array06 = [1, 2, 3, 4, 5];
    let vector06 = array06.to_vec();
    println!("array06: {:?}", array06);

    println!("\n... we can also use the `to_vec` method to convert a slice to a vector ...");
    let slice07 = &array06[1..4];
    let vector07 = slice07.to_vec();
    println!("slice07: {:?}", slice07);

    println!("==============================================================");
    println!("====                 Vectors To Arrays                    ====");
    println!("==============================================================");

    println!("... we can convert a vector to an array using the `as_slice` method ...");
    let vector08 = vec![1, 2, 3, 4, 5];
    let slice08 = vector08.as_slice();
    println!("vector08: {:?}", vector08);
    println!("slice08: {:?}", slice08);

    println!("==============================================================");
    println!("====                  Sorting Vectors                     ====");
    println!("==============================================================");

    println!("... we can sort a vector using the `sort` method ...");

    let mut vector09 = vec![5, 3, 1, 2, 4];
    vector09.sort();
    println!("vector09: {:?}", vector09);

    println!("==============================================================");
    println!("====                  Vector Methods                     ====");
    println!("==============================================================");

    println!("... we can use the `pop` method to remove the last element from a vector ...");

    let mut vector10 = vec![1, 2, 3, 4, 5];
    let popped = vector10.pop();

    match popped {
        Some(popped) => println!("The popped element is: {}", popped),
        None => println!("There is no element to pop"),
    }

    println!("... we can use the `push` method to add an element to the end of a vector ...");

    let mut vector11 = vec![1, 2, 3, 4, 5];
    vector11.push(6);
    println!("vector11: {:?}", vector11);

    println!("... we can use the `insert` method to add an element at a specific index in a vector ...");

    let mut vector12 = vec![1, 2, 3, 4, 5];
    vector12.insert(2, 6);
    println!("vector12: {:?}", vector12);

    println!("... we can use the `remove` method to remove an element at a specific index in a vector ...");

    let mut vector13 = vec![1, 2, 3, 4, 5];
    let removed = vector13.remove(2);

    match removed {
        Some(removed) => println!("The removed element is: {}", removed),
        None => println!("There is no element to remove"),
    }

    println!("... we can use the `len` method to get the number of elements in a vector ...");

    let vector14 = vec![1, 2, 3, 4, 5];
    let length = vector14.len();
    println!("The length of vector14 is: {}", length);

    println!("... we can use the `is_empty` method to check if a vector is empty ...");

    let vector15 = vec![1, 2, 3, 4, 5];
    let is_empty = vector15.is_empty();
    println!("Is vector15 empty? {}", is_empty);

    let vector16 = Vec::new();
    let is_empty = vector16.is_empty();

    println!("Is vector16 empty? {}", is_empty);

    println!("... we can use the `contains` method to check if a vector contains a specific element ...");

    let vector17 = vec![1, 2, 3, 4, 5];
    let contains = vector17.contains(&3);
    println!("Does vector17 contain 3? {}", contains);

    let contains = vector17.contains(&6);

    println!("Does vector17 contain 6? {}", contains);

    println!("... we can get the last item in a vector using the `last` method ...");

    let vector18 = vec![1, 2, 3, 4, 5];
    let last = vector18.last();

    match last {
        Some(last) => println!("The last element of vector18 is: {}", last),
        None => println!("There is no last element in vector18"),
    }

    println!("==============================================================");
    println!("====                    Using Enum                        ====");
    println!("==============================================================");

    println!("... we can use an enum to store multiple types in a vector ...");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }

    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Strings                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Strings are a collection of characters.");
    println!("They are always UTF-8 encoded.");
    println!("... one byte for ASCII, two bytes for most other characters, three for some Chinese characters, four for emojis etc ...");
    println!("They are stored on the heap rather than the stack.");
    println!("They are mutable.");
    println!("They are useful when you want to manipulate text.");
    println!("You can create a new, empty string with the String::new function.");
    println!("You can also create a string from a string literal with the to_string method.\n");
    println!("... take care to be aware of the differences between a `String` type and a `&str` string slice type ...");
    println!("... to prevent problems with different UTF8 character lengths, Rust does not allow you to index characters inside a string ...");
    println!("... grapheme clusters are the closest thing to what we think of as letters ... but can be of variable length ...");
    println!("... you can use the `chars` method to iterate over the characters in a string ...");
    println!("... you can use the `bytes` method to iterate over the bytes in a string ...");
    println!("... you can use the `len` method to get the number of bytes in a string ...");
    println!("... you can use the `is_empty` method to check if a string is empty ...");
    println!("... you can use the `contains` method to check if a string contains a substring ...");
    println!("... you can use the `replace` method to replace a substring with another substring ...");
    println!("... you can use the `split` method to split a string into parts based on a separator ...");
    println!("... you can use the `split_whitespace` method to split a string into words ...");
    println!("... you can use the `to_uppercase` method to convert a string to uppercase ...");
    println!("... you can use the `to_lowercase` method to convert a string to lowercase ...");
    println!("... you can use the `starts_with` method to check if a string starts with a substring ...");
    println!("... you can use the `ends_with` method to check if a string ends with a substring ...");
    println!("... you can use the `trim` method to remove whitespace from the beginning and end of a string ...");
    println!("... you can use the `trim_start` method to remove whitespace from the beginning of a string ...");
    println!("... you can use the `trim_end` method to remove whitespace from the end of a string ...");
    println!("... you can use the `parse` method to convert a string to a number ...");
    println!("... you can use the `format!` macro to concatenate strings without taking ownership ...");
    println!("... you can use the `push_str` method to append a string slice to a string ...");
    println!("... you can use the `push` method to append a single character to a string ...");
    println!("... you can use the `+` operator to concatenate strings ...");

    println!("==============================================================");
    println!("====                 Simple Examples                      ====");
    println!("==============================================================");

    let string01 = String::new();

    let string02 = "initial contents".to_string();

    let string03 = String::from("initial contents");

    println!("string01: {}", string01);
    println!("string02: {}", string02);
    println!("string03: {}", string03);

    println!("\n... we can append to a string using the push_str method ...");
    println!("... this method takes a string slice as a parameter ...\n");

    let mut string04 = String::from("foo");
    
    string04.push_str(" bar");

    println!("string04: {}", string04);

    let string05 = " baz";
    string04.push_str(string05);

    println!("string04: {}", string04);

    println!("\n... we can append a single character to a string using the push method ...");

    let mut string06 = String::from("lo");
    string06.push('l');

    println!("string06: {}", string06);

    println!("\n... we can concatenate strings using the + operator ...");

    let string07 = String::from("Hello, ");
    let string08 = String::from("world!");

    let string09 = string07 + &string08;

    println!("string09: {}", string09);

    println!("\n... note that the + operator takes ownership of the first string ...");
    println!("... so we cannot use string07 after the concatenation ...");
    println!("... also the reference to string08 is used as a slice and so string08 is still valid after the concatenation ...\n");

    println!("\n... we can use the format! macro to concatenate strings without taking ownership ...");

    let string10 = String::from("tic");
    let string11 = String::from("tac");
    let string12 = String::from("toe");

    let string13 = format!("{}-{}-{}", string10, string11, string12);

    println!("string13: {}", string13);

    println!("==============================================================");
    println!("====                    Slicing                          ====");
    println!("==============================================================");

    println!("... we can slice a string using square brackets and a range ...");

    let string14 = String::from("hello world");

    let string15 = &string14[0..5];

    println!("string15: {}", string15);

    let string16 = &string14[6..11];

    println!("string16: {}", string16);

    println!("... note that the range is exclusive of the end index ...");

    println!("==============================================================");
    println!("====                    Iterating                         ====");
    println!("==============================================================");

    println!("... we can iterate over the characters in a string using the chars method ...");

    for character in "नमस्ते".chars() {
        print!("{}   ", character);
    }

    println!("\n\n... we can iterate over the bytes in a string using the bytes method ...");

    for byte in "नमस्ते".bytes() {
        print!("{}   ", byte);
    }

    println!("\n\n... repeat using a simpler string ...");

    for character in "hi".chars() {
        print!("{}   ", character);
    }

    println!("\n\n... now printing the bytes ...");

    for byte in "hi".bytes() {
        print!("{}   ", byte);
    }

    println!("\n\n... now print individual characters ... ");
    
    for character in "abcdefg".chars() {
        print!("{}   ", character);
    }

    println!("\n\n... and the corresponding bytes ...");

    for byte in "abcdefg".bytes() {
        print!("{}   ", byte);
    }

    println!("\n");
    println!("==============================================================");
    println!("==============================================================");
    println!("====                    Hashmaps                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("\n==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("\nHashmaps store a collection of key-value pairs.");
    println!("The keys must be unique.");
    println!("... we write a hashmap as HashMap<K, V> where K = key type and V = value type ...");
    println!("... same as a dictionary in other languages ...");
    println!("They are stored on the heap rather than the stack.");
    println!("They are useful when you want to associate one value with another.");
    println!("You can create a new, empty hashmap with the HashMap::new function.");
    println!("You can add key-value pairs to it with the insert method.");
    println!("You can also create a hashmap that has the same value for each key with the hashmap! macro.\n");
    println!("When a collection goes out of scope rust drops the collection and all of the individual elements\n");

    println!("==============================================================");
    println!("====                 Simple Examples                      ====");
    println!("==============================================================");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    println!("\nto read a value from a hashmap we use the get method ...");
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(score) => println!("The score for the Blue team is: {}", score),
        None => println!("There is no score for the Blue team"),
    }

    println!("\n... we can also use the `unwrap_or` method to provide a default value ...");

    let score = scores.get(&team_name).unwrap_or(&0);

    println!("The score for the Blue team is: {}", score);

    println!("\n... we can iterate over the key-value pairs in a hashmap using a for loop ...");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("\nto return a boolean if a key exists in a hashmap we use the contains_key method ...");

    if scores.contains_key(&team_name) {
        println!("The Blue team has a score");
    } else {
        println!("The Blue team does not have a score");
    }

    println!("... another way of creating a hash map is to merge two collections into one ...");

    let teams_vector = vec![String::from("Blue"), String::from("Yellow")];
    let scores_vector = vec![10, 50];

    println!("teams_vector: {:?}", teams_vector);
    println!("scores_vector: {:?}", scores_vector);

    let scores_hashmap: HashMap<_, _> = teams_vector.iter().zip(scores_vector.iter()).collect();
    println!("scores_hashmap: {:?}", scores_hashmap);

    println!("==============================================================");
    println!("====                   Ownership                          ====");
    println!("==============================================================");

    println!("just be aware the same rules of ownership apply when adding objects to a collection");
    println!("... the items are effectively moved into the collection ...");
    println!("... so if you want to use the item after adding it to a collection you need to clone it ...");

    let key = String::from("Red");
    let value = 25;

    let mut scores = HashMap::new();
    scores.insert(key, value);

    println!("key is no longer available for use as it has been moved into the hashmap and the original item removed");
    println!("value: {}", value);

    println!("... to avoid this we can clone the key and value before adding them to the hashmap ...");

    let key = String::from("Red");
    let value = 25;

    let mut scores = HashMap::new();

    scores.insert(key.clone(), value);

    println!("key is still available for use as it has been cloned before adding it to the hashmap");
    println!("key: {}", key);
    println!("value: {}", value);

    println!("==============================================================");
    println!("====                    Updating                          ====");
    println!("==============================================================");

    println!("\n... we can update a value in a hashmap using the insert method ...");
    println!("... insert adds or overwrites the key and value ...");

    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);

    println!("\n... `entry` will only add a value if the key does not exist.");
    println!("... it will not overwrite an existing key or update the value");
    println!("... so in this example `blue` is ignored but `green` is added ...");

    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);

    println!("scores: {:?}", scores);

    println!("\n==============================================================");
    println!("====     Example Updating A Hashmap based on word count   ====");
    println!("==============================================================");

    println!("\n... in this example we continually update the hashmap count for each word");

    let text = "hello world wonderful world";
    let words = text.split_whitespace();

    println!("\n... text: {}", text);
    println!("... words: {:?}", words);

    let mut word_count_hashmap = HashMap::new();

    println!("\n... we create a new hashmap to store the word count ...");
    println!("... remember the 'entry' keyword will only add a new value and will not overwrite an existing value ...");
    for word in words {
        let word_count = word_count_hashmap.entry(word).or_insert(0);
        *word_count += 1;
    }

    println!("\nword count hashmap is : {:?}", word_count_hashmap);

    println!("\n... we see that each loop we check if an entry exists with `.entry` and if not, inserts a new value 0");
    println!("... we then increment the scalar value by 1");

    println!("\n==============================================================");
    println!("====                 Worked Examples                      ====");
    println!("==============================================================");

    println!("... we can use a hashmap to store a list of employees and their departments ...");

    let mut employees = HashMap::new();

    employees.insert(String::from("John"), String::from("Sales"));
    employees.insert(String::from("Jane"), String::from("Engineering"));
    employees.insert(String::from("Jim"), String::from("Marketing"));

    println!("employees: {:?}", employees);

    println!("\n==============================================================");
    println!("====                 Worked Exercises                      ====");
    println!("==============================================================");
        
    println!("\n... from the docs ...");

    println!("\nGiven a list of integers, use a vector
        return the median (when sorted, the value in the middle position)
        and mode (the value that occurs most often; 
        a hash map will be helpful here) of the list.");

    println!("\nConvert strings to pig latin. 
        The first consonant of each word is moved to the end of the word and ay is added, 
            so first becomes irst-fay. 
        Words that start with a vowel have hay added to the end instead 
            (apple becomes apple-hay). 
        Keep in mind the details about UTF-8 encoding!");

    println!("\nUsing a hash map and vectors, 
        create a text interface to allow a user to add employee names to a department in a company; 
        for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
        Then let the user retrieve a list of 
            all people in a department or 
            all people in the company by department, 
        sorted alphabetically");

    println!("\n==============================================================");
    println!("==============================================================");
    println!("====                     Summary                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("... vectors are useful when you have a list of items that you want to store in a single variable ...");
    println!("... strings are useful when you want to manipulate text ...");
    println!("... hashmaps are useful when you want to associate one value with another ...");

    println!("==============================================================");
    println!("==============================================================");
    println!("====                     End                              ====");
    println!("==============================================================");
    println!("==============================================================");

}
