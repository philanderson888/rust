use std::fmt;
fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                         OOP                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("... OOP uses classes and objects to organize code ...");

    println!("... encapsulation hides the internal state of an object ...");
    println!("... inheritance allows a class to inherit properties and methods from another class ...");
    println!("... polymorphism allows a method to do different things based on the object it is acting upon ...");
    println!("... abstraction allows a class to provide a simple interface to a complex object ...");

    println!("\n... In Rust, we can use structs and traits to achieve OOP concepts ...");
    println!("... we use structs and enums to define types and impl blocks to define methods ...");
    println!("... also rust does not have direct inheritance but uses traits which are collections of methods and are applied to types ...");
    println!("... also rust does not have constructors but can use an associated 'new' function to achieve the same thing ...");
    println!("... with encapsluation, fields are private by default and explicitly require the 'pub' keyword to be public ...");
    println!("... or use public getters and setters to access the fields ...");
    
    println!("\n... Let's look at an example of encapsulation ...");

    println!("==============================================================");
    println!("====                    Encapsulation                     ====");
    println!("==============================================================");

    println!("... if we look at the example we see the struct is public but the fields are private ...");
    println!("... to access the fields we have to use the public getters and setters which ensure ...");
    println!("... the fields are accessed in a controlled way ...");

    let mut collection = AveragedCollection {
        list: vec![],
        average: 0.0,
    };

    collection.add(5);
    collection.add(10);
    collection.add(15);

    println!("collection is {}", collection);
    collection.print_collection();

    println!("==============================================================");
    println!("====                    Inheritance                       ====");
    println!("==============================================================");

    println!("... Inheritance is a way to form new classes using classes that have already been defined ...");
    println!("... The new classes, known as derived classes, inherit attributes and behavior of the pre-existing classes, which are referred to as base classes ...");

    println!("... In Rust, we can use traits to achieve inheritance ...");

    println!("==============================================================");
    println!("====                    Polymorphism                      ====");
    println!("==============================================================");

    println!("... Polymorphism is the ability to present the same interface for different data types ...");
    println!("... Polymorphism allows methods to do different things based on the object it is acting upon ...");

    println!("... In Rust, we can use traits to achieve polymorphism ...");

    println!("==============================================================");
    println!("====                    Abstraction                       ====");
    println!("==============================================================");

    println!("... Abstraction is the concept of object-oriented programming that ");
    println!("... shows only essential attributes and hides unnecessary information ...");

    println!("... In Rust, we can use traits to achieve abstraction ...");

}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    fn print_collection(&self) {

        let mut collection_string = String::from("collection is : ");
        for value in &self.list {
            collection_string.push_str(&format!("{}, ", value));
        }
        // Remove the trailing comma and space
        if !collection_string.is_empty() {
            collection_string.pop();
            collection_string.pop();
        }
        println!("{}", collection_string);
    }
}

impl fmt::Display for AveragedCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AveragedCollection {{ list: {:?}, average: {:.2} }}", self.list, self.average)
    }
}


