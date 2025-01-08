use std::fmt::Display;
fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Generics                         ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("generics use the type <T> to represent any valid type in a system");
    println!("... we will be learning about traits which define interfaces which are implemented using the impl keyword ...");
    println!("... we also will be learning about lifetimes for references");

    println!("\n... a generic type is any type represented by the type <T>");
    println!("... a concrete type is a real fixed type eg integer, float, string etc");

    println!("\n... at runtime, Rust will replace the generic type with the concrete type");
    println!("... this is called monomorphization");

    println!("==============================================================");
    println!("====                   Simple Example                     ====");
    println!("==============================================================");

    println!("... we will be creating a function that will return the largest number in a list");
    println!("... we will start by creating a function that will return the largest number in a list without using generics");  

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest_number_without_generics(number_list);
    println!("... the largest number is {}", largest_number);

    println!("... if we repeat this again with a different list ... ");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest_number = get_largest_number_without_generics(number_list);
    println!("... the largest number is {}", largest_number);

    println!("... if we pass in references to the list ... ");

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest_number_without_generics_by_reference(&number_list);
    println!("... the largest number is {}", largest_number);

    println!("... if we repeat this again with a different list ... ");
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let largest_number = get_largest_number_without_generics_by_reference(&number_list);
    println!("... the largest number is {}", largest_number);

    println!("\n\nnow imagine that we also want to get the largest character in a list of characters");
    println!("... we will have to create another function that will return the largest character in a list without using generics");

    let character_list = vec!['y', 'm', 'a', 'q', 'z', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let largest_character = get_largest_character_without_generics(character_list);
    println!("... the largest character is {}", largest_character);

    println!("... if we repeat this again with a different list ... ");
    let character_list = vec!['y', 'm', 'a', 'q', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'o', 'p', 'r', 's', 't', 'u', 'v', 'w', 'x'];
    let largest_character = get_largest_character_without_generics(character_list);
    println!("... the largest character is {}", largest_character);

    println!("\n... if we pass in references to the list ... ");
    let character_list = vec!['y', 'm', 'a', 'q', 'z', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let largest_character = get_largest_character_without_generics_by_reference(&character_list);
    println!("... the largest character is {}", largest_character); 

    println!("... if we repeat this again with a different list ... ");
    let character_list = vec!['y', 'm', 'a', 'q', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'o', 'p', 'r', 's', 't', 'u', 'v', 'w', 'x'];
    let largest_character = get_largest_character_without_generics_by_reference(&character_list);
    println!("... the largest character is {}", largest_character);

    println!("... as we can see, we have to create a new function for each type we want to get the largest value for");
    println!("... this is not efficient and can be avoided by using generics");

    println!("... we can also simplify matters further by passing in a slice rather than a vector, let's show a simple example of this ...");

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest_number_using_slice(&number_list);
    println!("... the largest number is {}", largest_number);

    println!("... and the character version ...");

    let character_list = vec!['y', 'm', 'a', 'q', 'z', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let largest_character = get_largest_character_using_slice(&character_list);
    println!("... the largest character is {}", largest_character);

    println!("==============================================================");
    println!("====                   Using Generics                     ====");
    println!("==============================================================");

    println!("... we create a function that will return the largest value in a list using generics");
    println!("... we will use the PartialOrd trait to ensure that the values in the list can be compared");

    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = get_largest_value(&number_list);
    println!("... the largest number is {}", largest_number);

    println!("\n... we can now use the same function to get the largest character in a list of characters");
    
    let character_list = vec!['y', 'm', 'a', 'q', 'z', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let largest_character = get_largest_value(&character_list);
    println!("... the largest character is {}", largest_character);

    println!("... we can also use the same function to get the largest value in a list of floats");

    let float_list = vec![34.0, 50.0, 25.0, 100.0, 65.0];
    let largest_float = get_largest_value(&float_list);
    println!("... the largest float is {}", largest_float);

    println!("... we can also use the same function to get the largest value in a list of strings");

    let string_list = vec!["hello", "world", "this", "is", "a", "test"];
    let largest_string = get_largest_value(&string_list);
    println!("... the largest string is {}", largest_string);

    println!("... we can also use the same function to get the largest value in a list of tuples");

    let tuple_list = vec![(1, 2), (3, 4), (5, 6), (7, 8), (9, 10)];
    let largest_tuple = get_largest_value(&tuple_list);
    println!("... the largest tuple is {:?}", largest_tuple);

    println!("... we can also use the same function to get the largest value in a list of arrays");

    let array_list = vec![[1, 2], [3, 4], [5, 6], [7, 8], [9, 10]];
    let largest_array = get_largest_value(&array_list);
    println!("... the largest array is {:?}", largest_array);

    println!("... so we see how versatile such a function can be, with the minimum of code we are performing very powerful operations on completely different types!");

    println!("==============================================================");
    println!("====              Generics In Structs                     ====");
    println!("==============================================================");

    println!("... we can also use generics in structs, let's create a struct that will hold a value of any type");

    let integer_point = Point { x: 5, y: 10 };
    println!("... the x value is {}", integer_point.x);
    println!("... the y value is {}", integer_point.y);

    let float_point = Point { x: 5.0, y: 10.0 };
    println!("... the x value is {}", float_point.x);
    println!("... the y value is {}", float_point.y);

    println!("... we can also use generics in enums, let's create an enum that will hold a value of any type");

    let some_integer = Option::Some(5);
    let some_float = Option::Some(5.0);
    let some_string = Option::Some("hello");

    println!("... the integer value is {:?}", some_integer);
    println!("... the float value is {:?}", some_float);
    println!("... the string value is {:?}", some_string);

    println!("... we can also use the None variant to represent a value that is not present");

    let none_integer : Option<i32> = Option::None;
    let none_float : Option<f32> = Option::None;
    let none_string : Option<&str> = Option::None;

    println!("... the integer value is {:?}", none_integer);
    println!("... the float value is {:?}", none_float);
    println!("... the string value is {:?}", none_string);

    println!("==============================================================");
    println!("====             Generics With Different Types            ====");
    println!("==============================================================");

    println!("... with our struct point imagine the x and y axis are of different types");

    let point = PointWithDifferentTypes { x: 5, y: 10.0 };
    println!("... the x value is {}", point.x);
    println!("... the y value is {}", point.y);

    println!("==============================================================");
    println!("====            Generics In Methods                       ====");
    println!("==============================================================");

    println!("... we can also use generics in methods, let's create a method that will return the x value of a point");

    let integer_point = Point { x: 5, y: 10 };
    println!("... we have point {:?}", integer_point);
    println!("... the x value is {}", integer_point.x());

    println!("... this method will work for any type of point, for example a float point");

    let float_point = Point { x: 5.0, y: 10.0 };
    println!("... we have point {:?}", float_point);
    println!("... the x value is {}", float_point.x());

    println!("==============================================================");
    println!("====               Constraining Generics                  ====");
    println!("==============================================================");

    println!("... we can constrain generics depending on the type ... ");
    println!("\n... for example with our point struct we can specify for floats we have an extra function available");

    let float_point = Point { x: 5.0, y: 10.0 };
    println!("... we have point {:?}", float_point);
    println!("... the distance from the origin is {}", float_point.distance_from_origin());

    println!("... if we try to use this method with an integer point we will get an error");

    let integer_point = Point { x: 5, y: 10 };
    println!("... we have point {:?}", integer_point);
    // println!("... the distance from the origin is {}", integer_point.distance_from_origin());

    println!("==============================================================");
    println!("====                    Traits                            ====");
    println!("==============================================================");

    println!("... traits are interfaces which are implemented using the impl keyword ... ");

    println!("\n... eg Summary trait with method summarize()");
    println!("... this method inputs itself (any object) and returns a string");

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };

    println!("... the news article is {:?}", news_article);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("... the tweet is {:?}", tweet);

    println!("... we can now call the summarize method on the news article and tweet");

    println!("... the news article summary is {}", news_article.summarize());
    println!("... the tweet summary is {}", tweet.summarize());

    println!("==============================================================");
    println!("====                 Default Traits                       ====");
    println!("==============================================================");

    println!("... we can also provide default implementations for traits");

    println!("... we will create a trait called SummaryWithDefault that will have a method called summarize");

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };

    println!("... the news article is {:?}", news_article);
    println!("... the news article summary is {}", news_article.summarize_with_default());

    println!("... we can also call the summarize_author method on the tweet");

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };

    println!("... the tweet is {:?}", tweet);

    println!("... the tweet summary is {}", tweet.summarize());
    println!("... the tweet author summary is {}", tweet.summarize_author());

    println!("==============================================================");
    println!("====                 Traits As Parameters                 ====");
    println!("==============================================================");
    println!("... we can also use traits as parameters in functions");
    println!("... we will create a function called notify that will take any object that implements the Summary trait");
    println!("... the syntax reads ... fn notify(item: &impl Summary) ...");

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),  
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };

    println!("... the news article is {:?}", news_article);

    notify(&news_article);

    println!("... the tweet is {:?}", tweet);

    notify(&tweet);

    println!("==============================================================");
    println!("====                 Multiple Traits                      ====");
    println!("==============================================================");
    println!("... a type can implement multiple traits ...");
    println!("\n... eg create function notify_and_display - takes any object which implments Summary and Display traits");
    notify_and_display(&news_article);

    println!("==============================================================");
    println!("====                  Where Syntax                        ====");
    println!("==============================================================");
    println!("... we can also use the where syntax to constrain the types that can be used in a function");
    println!("... the syntax reads ... fn notify_and_display_with_where_syntax<T>(item: &T) where T: Summary + Display ...");
    notify_and_display_with_where_syntax(&news_article);

    println!("==============================================================");
    println!("====                Returning Traits                      ====");
    println!("==============================================================");
    println!("... functions can return traits so we are sure of the behaviour of any types returned from the function ... ");
    println!("\n... we will create a function called returns_summarizable that will return an object that implements the Summary trait");
    println!("... the syntax reads ... fn returns_summarizable() -> impl Summary ...");

    let tweet = returns_summarizable();
    println!("\n... the tweet returned via the returned trait function is summarizable ... ");

    println!("\n... the tweet summary is {}", tweet.summarize());
    println!("... the tweet author summary is {}", tweet.summarize_author());

    println!("==============================================================");
    println!("====               Conditional Traits                     ====");
    println!("==============================================================");
    println!("... we can also use conditional traits to constrain the types that can be used in a function");
    println!("... example syntax will be ... fn some_function<T: Display + Clone>(t: T) ...");

    println!("==============================================================");
    println!("====              Blanket Implementations                 ====");
    println!("==============================================================");
    println!("... we can also implement traits for any type that implements a specific trait");
    println!("... for example we can implement the Display trait for any type that implements the Summary trait");
    println!("... syntax would be ... impl<T: Summary> Display for T ...");

    println!("==============================================================");
    println!("====                    Lifetimes                         ====");
    println!("==============================================================");

    println!("... lifetimes are a way to ensure that references are valid for a certain period of time");
    println!("... lifetimes are not use in other languages so can be confusing at first");
    println!("... the main aim of lifetimes is to prevent a) dangling references b) use-after-free errors");
    println!("... a dangling reference is a reference to a value that no longer exists");
    println!("... a use-after-free error is when we try to use a reference to a value that has been deallocated");
    println!("... a lifetime is a pointer and length to any string in memory");
    println!("... c programs will compile if a potential bug exists but rust programs will not ... ");
    println!("... when we use a reference eg in a struct, we need to ensure that the reference is valid for the lifetime of the struct");
    println!("... we can use lifetimes to ensure that the reference is valid for the lifetime of the object it references");
    println!("... we need to use lifetimes when we have references in structs or functions");
    println!("... lifetimes are usually denoted by an apostrophe eg 'a");
    println!("... lifetimes on the inputs are called input lifetimes");
    println!("... lifetimes on the outputs are called output lifetimes");
    println!("... lifetime elision rules are used to determine the lifetimes of references automatically ... ");
    println!("... these elision rules dont always require us to specify lifetimes");
    println!("... static lifetime is valid for the entire duration of the program");
    println!("... lifetimes are the way to be memory safe without garbage collection");

    let r;

    {
        let x = 5;
        r = &x;
        println!("... the value of x is {}", x);
        println!("... the value of r is {}", r);
    }

    // println!("... the value of r is {}", r);

    println!("... we can use lifetimes to ensure that the reference r is valid for the lifetime of x");

    let x = 5;
    let result = &x;
    println!("... the value of x is {}", result);

    println!("==============================================================");
    println!("====              Lifetimes In Functions                  ====");
    println!("==============================================================");

    println!("... we can also use lifetimes in functions");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // as_str extracts a string slice from a string ... so we are passing two slices into this function
    let result = longest(string1.as_str(), string2);

    println!("... the longest string is {}", result);

    println!("... the lifetime parameters must start with an apostrophe and are usually called 'a");
    println!("... &'a means that the reference is valid for the lifetime of 'a");
    println!("... &'a mut means that the mutable reference is valid for the lifetime of 'a");

    println!("==============================================================");
    println!("====              Lifetime Annotations                    ====");
    println!("==============================================================");

    println!("... we can also use lifetime annotations to specify the lifetime of a reference");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("... the longest string is {}", result);
    }

    println!("... the lifetime of string1 is longer than string2 so we can use string1 in the longest function");

    println!("==============================================================");
    println!("====              Lifetime Annotations In Structs         ====");
    println!("==============================================================");

    println!("... we can also use lifetime annotations in structs");

    let _string1 = String::from("long string is long");

    let result;
    {
        let string2 = String::from("xyz");
        let important_excerpt = ImportantExcerpt { part: string2.as_str() };
        result = important_excerpt;
        println!("... the important excerpt is {:?}", result);
        println!("... result.part is {}", result.part);
    }

    println!("... the lifetime of string1 is longer than string2 so we can use string1 in the important excerpt struct");

    println!("==============================================================");
    println!("====             Combining Generics And Lifetimes         ====");
    println!("==============================================================");

    println!("... we can also combine generics and lifetimes");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest_with_annotations(string1.as_str(), string2.as_str());
    println!("... the longest string is {}", result);

    println!("... we can also use lifetime annotations in functions");

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "the longest string is");
    println!("... the longest string is {}", result);

}

fn get_largest_number_without_generics(number_list : Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_number_without_generics_by_reference(number_list : &Vec<i32>) -> &i32 {
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_number_using_slice(number_list : &[i32]) -> &i32 {
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn get_largest_character_without_generics(character_list : Vec<char>) -> char {
    let mut largest = character_list[0];
    for character in character_list {
        if character > largest {
            largest = character;
        }
    }
    largest
}

fn get_largest_character_without_generics_by_reference(character_list : &Vec<char>) -> &char {
    let mut largest = &character_list[0];
    for character in character_list {
        if character > largest {
            largest = character;
        }
    }
    largest
}

fn get_largest_character_using_slice(character_list : &[char]) -> &char {
    let mut largest = &character_list[0];
    for character in character_list {
        if character > largest {
            largest = character;
        }
    }
    largest
}

fn get_largest_value<T: PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];
    for value in list {
        if value > largest {
            largest = value;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct PointWithDifferentTypes<T, U> {
    x: T,
    y: U
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        String::from("(Read more from our favourite author ...)")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub trait DisplayArticle {
    fn display(&self) -> String;
}

impl DisplayArticle for NewsArticle {
    fn display(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub trait SummaryWithDefault {
    fn summarize_with_default(&self) -> String {
        String::from("(Read more...)")
    }
}

impl SummaryWithDefault for NewsArticle {}

// passing in the summary trait as a parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_and_display(item: &(impl Summary + DisplayArticle)) {
    println!("Breaking news! {}", item.summarize());
    println!("Displaying news! {}", item.display());
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify_and_display_with_where_syntax<T>(item: &T) where T: Summary + DisplayArticle {
    println!("Breaking news! {}", item.summarize());
    println!("Displaying news! {}", item.display());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

fn longest_with_annotations<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
