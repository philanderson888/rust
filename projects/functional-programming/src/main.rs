use std::time::Duration;
use std::thread;
fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====               Functional Programming                 ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Functional programming is a programming paradigm ...");
    println!("... it treats computation as the evaluation of mathematical functions and avoids changing-state and mutable data");

    println!("\n... it is a declarative type of programming style");
    println!("\n... the topics covered here are ... ");

    println!("\n... 1. Closures: anonymous functions that can capture their environment");
    println!("\n... 2. Iterators: a way of processing a series of items");
    println!("\n... 3. The Iterator Trait: a way to define iterators");
    println!("\n... 4. Using Iterator Trait Methods: a way to use the Iterator trait methods");

    println!("==============================================================");
    println!("====                   Closures                          ====");
    println!("==============================================================");

    println!("Closures are anonymous functions that can capture their environment");
    println!("... they can capture values from the scope in which they are defined");

    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));


    println!("==============================================================");
    println!("====                   Iterators                         ====");
    println!("==============================================================");

    println!("Iterators are a way of processing a series of items");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("==============================================================");
    println!("====                   The Iterator Trait                ====");
    println!("==============================================================");

    println!("The Iterator trait is used to implement iterators over collections such as vectors");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));

    assert_eq!(v1_iter.next(), Some(&2));

    assert_eq!(v1_iter.next(), Some(&3));

    assert_eq!(v1_iter.next(), None);

    println!("==============================================================");
    println!("====                   Using Iterator Trait Methods      ====");
    println!("==============================================================");

    println!("The Iterator trait has a number of methods with default implementations");

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    println!("==============================================================");
    println!("====                   Example                           ====");
    println!("==============================================================");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    println!("==============================================================");
    println!("====                   Closure Example                    ====");
    println!("==============================================================");

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let _expensive_result = expensive_closure(42);

    println!("==============================================================");
    println!("====                   Closure Syntax                     ====");
    println!("==============================================================");

    println!("Closure syntax is similar to function syntax");

    println!("\n... firstly one function ... ");
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }

    println!("... followed by three closures that do the same thing");
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    println!("... the closures are called with the same value");

    let x = 1;
    println!("add_one_v1(x) = {}", add_one_v1(x));
    println!("add_one_v2(x) = {}", add_one_v2(x));
    println!("add_one_v3(x) = {}", add_one_v3(x));
    println!("add_one_v4(x) = {}", add_one_v4(x));

    println!("\nnote that once the closure has inferred the types, they are fixed and cannot be changed");

    println!("==============================================================");
    println!("====                    Immutable References              ====");
    println!("==============================================================");

    println!("... if it is possible closures will capture values by immutable reference");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!("\n... we see that in this example both 'list' and 'only_borrows' are immutable and references are passed");

    println!("==============================================================");
    println!("====                    Mutable References                ====");
    println!("==============================================================");

    println!("... we now show an example where mutable references are passed");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    println!("==============================================================");
    println!("====                    Moving Values                     ====");
    println!("==============================================================");

    println!("... we now show an example where values are moved");
    println!("... in this case we can move an item from one thread to another");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    println!("... list is now out of scope as has been moved to a background thread, so cannot be called from the ui thread");

    println!("==============================================================");
    println!("====             Moving Values Out Of Closures            ====");
    println!("==============================================================");

    println!("... closures have one of three options when moving values out of the closure");
    println!("... 1. FnOnce: consumes the variables it captures from its enclosing scope (moving items)");
    println!("... 2. FnMut: mutably borrows values from the enclosing scope (passing a mutable reference)");
    println!("... 3. Fn: borrows values from the enclosing scope (passing an immutable reference)");

    println!("... 1. example is Option<T> function unwrap_or_else which can only be called once ...");
    println!("... 2. example is Vec<T> function push which can be called multiple times ...");
    println!("... 2. example is list sort_by_key which can be called multiple times ...");

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
    println!("... the list has been sorted by width");
    println!("... we can still read the list");

    println!("list first element is {} x {}", list[0].width, list[0].height);

    println!("==============================================================");
    println!("====                     Iterators                        ====");
    println!("==============================================================");

    println!("... iterators are a way of processing a series of items");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    println!("... we can for example use an iterator to sum the items in a list");

    let v1 = vec![1, 2, 3];

    let total: i32 = v1.iter().sum();

    println!("vector v1 is {:?}", v1);
    println!("The total is: {}", total);


    println!("==============================================================");
    println!("====                     The Iterator Trait               ====");
    println!("==============================================================");

    println!("... the Iterator trait is used to implement iterators over collections such as vectors");

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));


    println!("==============================================================");
    println!("====            Using Iterator Trait Methods              ====");
    println!("==============================================================");

    println!("... the Iterator trait has a number of methods with default implementations");

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    
    println!("==============================================================");
    println!("====                Iterator Adapters                     ====");
    println!("==============================================================");

    println!("... iterator adapters are methods that allow you to change iterators into different kinds of iterators");

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    println!("... v1 is {:?}", v1);
    println!("... v2 is {:?}", v2);

    println!("... the map method is an iterator adapter that is used to create a new iterator by calling a closure on each item");
    println!("... iterators by default are 'lazy' so to get output for example in this instance we use the collect method");

    println!("==============================================================");
    println!("====        Passing Closures As Arguments Into Iterators  ====");
    println!("==============================================================");

    println!("... we can pass closures as arguments into iterators");

    println!("... we can use shoe size as an example ... ");

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("... shoes in my size are {:?}", in_my_size);

    println!("... the shoes_in_size function iterates over the shoes vector and matches the shoe size to ... ");
    println!("... the closure captures the shoe size from the environment ... ");

    println!("==============================================================");
    println!("====                   Summary So Far                     ====");
    println!("==============================================================");

    println!("... functional programming is a programming paradigm ...");
    println!("... it treats computation as the evaluation of mathematical functions and avoids changing-state and mutable data");

    println!("... closures are anonymous functions that can capture their environment");
    println!("... they can capture values from the scope in which they are defined");

    println!("... iterators are a way of processing a series of items");

    println!("... the Iterator trait is used to implement iterators over collections such as vectors");

    println!("... the Iterator trait has a number of methods with default implementations");

    println!("... iterator adapters are methods that allow you to change iterators into different kinds of iterators");

    println!("... we can pass closures as arguments into iterators");

    println!("... closures are a powerful feature of Rust that allow you to write code that is more flexible and expressive");

    println!("... closures can capture their environment and use the values they capture");

    println!("==============================================================");
    println!("====                 Iterators vs Loops                   ====");
    println!("==============================================================");

    println!("... should we use iterators or loops? ... ");
    println!("... iterators are more concise and expressive than loops");
    println!("... iterators are also more flexible and can be used with a variety of different data types");
    
    println!("... loops are more verbose and less expressive than iterators");

    println!("==============================================================");
    println!("====                 Performance Considerations           ====");
    println!("==============================================================");

    println!("... iterators are generally faster than loops because they are more optimized by the compiler");

    println!("... iterators are also more flexible and can be used with a variety of different data types");

    println!("... check the benchmark in the book - iterators are slightly faster ...");

    println!("... iterators are known as a 'zero-cost abstraction' ... they compile down to the same machine code as loops");

    println!("... rust performs all checks at compile time so run time performance is not affected");


}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        println!("closure is a function that captures its environment...");
        println!("... it can capture values from the scope in which it is defined");
        println!("... || self.most_stocked() is a closure");
        println!("... it captures the self environment");
        println!("... self is the Inventory struct");
        println!("... note that closures do not have to state the input parameters types or the return type");
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}






#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
