use List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;

fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                   Smart Pointers                     ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("a regular pointer simply points to an address in memory");
    println!("a smart pointer has additional metadata and capabilities");
    println!("smart pointers are implemented using structs");
    println!("smart pointers implement the Deref and Drop traits");
    println!("... Deref allows a smart pointer to be treated like a regular pointer");
    println!("... Drop allows a smart pointer to clean up resources when it goes out of scope");
    
    println!("\n... in many cases smart pointers actually own the data they point to");
    println!("... this allows them to clean up the data when they go out of scope");
    println!("... this is unlike references which merely point to data owned by someone else");

    println!("\nsome smart pointers we have already encountered include");
    println!(" ... String");
    println!(" ... Vec<T>");

    println!("\nsmart pointers are used to implement");
    println!(" ... reference counting");
    println!(" ... interior mutability");
    println!(" ... memory management");
    println!(" ... concurrency");
    
    println!("\nBox<T> is an example of a smart pointer");
    println!("Box<T> typically wraps a primitive and places it on the heap");
    println!("Box<T> is used when you have a type whose size can't be determined at compile time");
    println!("Box<T> is used when you have a type that you want to have a known size");
    println!("Box<T> is used when you have a large amount of data and you want to transfer ownership but ensure the data is not copied");
    println!("Box<T> is used when you have a type that implements a trait but you don't know which type it is at compile time");
    println!("\nRc<T> is a reference counting smart pointer");
    println!("Rc<T> is used when you have multiple owners of a value");
    println!("Rc<T> tracks the number of references and cleans up the value when the number of references reaches zero");

    println!("\nRefCell<T> is a smart pointer that allows mutable borrows checked at runtime");
    println!("RefCell<T> is used when you have a type that you want to mutate but you have immutable references to it");
    println!("Ref<T> and RefMut<T> are used to access the data in a RefCell<T>");

    println!("\nThe interior mutability pattern is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data");
    println!("... this uses 'unsafe' to bend Rust's usual rules that govern mutation and borrowing");

    println!("\nreference cycles are also discussed as they can cause memory leaks");

    println!("\n\n==============================================================");
    println!("====                      Box<T>                          ====");
    println!("==============================================================");

    println!("Box<T> is a smart pointer that allows you to store data on the heap rather than the stack");

    let box01 = Box::new(5);

    println!("box01 = {}", box01);

    println!("... the box data eg '5' is wrapped and stored on the heap ... ");
    println!("... the box reference pointer is stored on the stack ... ");

    println!("\n... the box reference pointer is dropped when it goes out of scope ... ");

    println!("==============================================================");
    println!("====              Recursive Types        Rc<T>            ====");
    println!("==============================================================");

    println!("... with a regular type the compiler can work out the size of the type at compile time ... ");
    println!("... with a recursive type the compiler can't work out the size of the type at compile time ... ");
    println!("... this is because the type refers to itself ... ");

    println!("\n... here we have an example using a 'cons list' ... ");
    println!("... a cons list is a data structure that defines a list as either an element or a pair of an element and another list ... ");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);

    println!("\nHead:");
    match &list {
        Cons(head, tail) => {
            println!("{}", head);
            print_list(tail);
        },
        Nil => println!("Empty list"),
    }

    println!("==============================================================");
    println!("====                Deref Trait        Rc<T>              ====");
    println!("==============================================================");

    println!("... the Deref trait allows a smart pointer to be treated like a regular pointer ... ");

    println!("... the derefercing operator '*' is used to dereference a pointer ... ");

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("... the derefercing operator '*' is used to dereference a smart pointer ... ");
    println!("... this means that we look at the value pointed to rather than the pointer itself ... ");

    println!("... we could rewrite this as ... ");

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


    println!("... let's build another example again using a custom box type MyBox<T> holding any type ... ");

    println!("... to get it to compile we have to add the Deref trait to MyBox<T> ... ");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


    println!("==============================================================");
    println!("====             Dereferencing Coertion                   ====");
    println!("==============================================================");


    println!("... dereferencing coercion is a feature in Rust that allows you to treat a reference as a smart pointer ... ");
    println!("... this means that you can use a reference in place of a smart pointer ... ");

    println!("\n... eg a reference to a String can convert to a reference to a str ... ");
    println!("... this is because the Deref trait is implemented for String ... ");
    println!("... also a reference to a Vec<T> can convert to a reference to a slice ... ");
    println!("... this is because the Deref trait is implemented for Vec<T> ... ");
    println!("... also a reference to a Box<T> can convert to a reference to a T ... ");
    println!("... this is because the Deref trait is implemented for Box<T> ... ");


    println!("\n... here is an example ... ");
    println!("... in this example the MyBox is dereferenced to a String which is dereferenced to a str ... ");
    let dereference01 = MyBox::new(String::from("Rust"));
    hello(&dereference01);


    
    
    
    
    
    println!("==============================================================");
    println!("====             Mutable Derefercing Coertion             ====");
    println!("==============================================================");

    println!("... dereferencing coercion also works with mutable references ... ");

    println!("\n... eg a mutable reference to a Vec<T> can convert to a mutable reference to a slice ... ");

    println!("\n... here is an example ... ");

    let mut dereference01 = MyBox::new(String::from("Rust"));
    dereference01.0.push_str(" is great");
    hello(&dereference01);




    println!("==============================================================");
    println!("====                    Drop Trait                        ====");
    println!("==============================================================");

    println!("... the Drop trait is used to clean up resources when a smart pointer goes out of scope ... ");
    println!("... the Drop trait is used to implement the 'drop' method ... ");
    println!("... an example would be to drop resources like files or network connections ... ");
    println!("... drop is called automatically when a smart pointer goes out of scope ... ");
    println!("... drop is also called a 'destructor' in other programming languages ... ");
    println!("... this is the opposite of the 'constructor' ... ");

    let custom_smart_pointer_01 = CustomSmartPointer {
        data: String::from("pointer 01 ... created first ... dropped last"),
    };
    
    let custom_smart_pointer_02 = CustomSmartPointer {
        data: String::from("pointer 02 ... created last ... dropped first"),
    };

    println!("\nsmart pointers created ... \n");

    custom_smart_pointer_01.print();
    custom_smart_pointer_02.print();

    println!("\nsmart pointers dropped ... \n");

    println!("==============================================================");
    println!("====           Dropping Early with ::mem::drop            ====");
    println!("==============================================================");

    println!("... you can drop a smart pointer early using the ::mem::drop function ... ");

    let custom_smart_pointer_03 = CustomSmartPointer {
        data: String::from("smart pointer 03"),
    };
    println!("smart pointer 03 created");
    println!("... we call drop() manually and this calls .drop() automatically ... ");
    drop(custom_smart_pointer_03);
    println!("smart pointer 03 dropped prematurely");

    println!("==============================================================");
    println!("====                End Of Dropping Early                 ====");
    println!("==============================================================");


    println!("==============================================================");
    println!("====        RC ... pointers with a Reference Count        ====");
    println!("==============================================================");

    println!("... Rc<T> is a reference counting smart pointer ... ");
    println!("... Rc<T> allows multiple owners of a value ... ");
    println!("... Rc<T> tracks the number of references and cleans up the value when the number of references reaches zero ... ");

    println!("\n Rc<T> is useful when we don't know at compile time which owner will be the last to drop the value ...");

    println!("\n... also be aware Rc<T> is not thread safe ... so is to only be used in single threaded applications ...");

    println!("\n... here is an example ... ");

    println!("... scenario is two lists share the same ending from another list ... ");

    println!("... have to open in new project 'smart-pointers-rc' ... ");

    println!("==============================================================");
    println!("====                End Of RC Example                     ====");
    println!("==============================================================");

    println!("==============================================================");
    println!("====        RefCell<T> ... smart pointers with interior mutability        ====");
    println!("==============================================================");

    println!("... RefCell<T> is a smart pointer that allows mutable borrows checked at runtime ... ");
    println!("... RefCell<T> is used when you have a type that you want to mutate but you have immutable references to it ... ");
    println!("... RefCell<T> represents single ownership over the data it holds ... ");
    println!("... with traditional boxing the references are validated at compile time ... ");
    println!("... with RefCell<T> the references are only validated at runtime ... ");
    println!("... and any violation will cause a run time panic ... ");

    println!("... Ref<T> and RefMut<T> are used to access the data in a RefCell<T> ... ");

    println!("... this is called 'interior mutability' and uses the 'unsafe' flag to mutate data ... ");
    println!("... even though immutable references are held to that data ... ");

    println!("\n... basically it is bypassing all of the compiler's safe checking and saying 'hey, we have got this, relax ... '");

    println!("==============================================================");
    println!("====                    Summary So Far                    ====");
    println!("==============================================================");

    println!("... smart pointers are used to implement reference counting, interior mutability, memory management and concurrency ... ");

    println!("... Box<T> is a smart pointer that allows you to store data on the heap rather than the stack ... ");
    println!("... Box<T> is used when you have a type whose size can't be determined at compile time ... ");
    println!("... Box<T> is used when you have a type that you want to have a known size ... ");
    println!("... Box<T> is used when you have a large amount of data and you want to transfer ownership but ensure the data is not copied ... ");
    println!("... Box<T> is used when you have a type that implements a trait but you don't know which type it is at compile time ... ");
    println!("... Box<T> allows mutable or non-mutable references to the data ... ");

    println!("... Rc<T> is a reference counting smart pointer ... ");
    println!("... Rc<T> is used when you have multiple owners of a value ... ");
    println!("... Rc<T> tracks the number of references and cleans up the value when the number of references reaches zero ... ");

    println!("... RefCell<T> is a smart pointer that allows mutable borrows checked at runtime ... ");
    println!("... RefCell<T> is used when you have a type that you want to mutate but you have immutable references to it ... ");
    println!("... Ref<T> and RefMut<T> are used to access the data in a RefCell<T> ... ");


    println!("==============================================================");
    println!("====                End Of Summary So Far                 ====");
    println!("==============================================================");

    println!("==============================================================");
    println!("====        Interior Mutable Example with RefCell<T>      ====");
    println!("==============================================================");

    println!("... a use case for this might be a mock object in a unit test ... ");

    println!("... this example tracks the usage of a customer and sends them messages depending on the usage value ... ");
    println!("... this might be akin to the usage quote allocated to a user; we notify them as the usage progresses ... ");

    println!("... run the example as a unit test ... ");

    println!("... and for another example see 'smart-pointers-rc-2' ... ");

    println!("==============================================================");
    println!("====             Memory Leaks with Reference Cycles       ====");
    println!("==============================================================");

    println!("... a reference cycle is when two references point to each other ... ");
    println!("... this can cause memory leaks ... ");
    println!("... as the reference count never reaches zero ... ");
    println!("... and the memory is never cleaned up ... ");
    println!("... using Rc<T> and RefCell<T> we can create a scenario where memory is being leaked ... ");

    println!("\n... example to be worked through later ... ");


    println!("\n\n\n");
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list: &List) {
    match list {
        Cons(head, tail) => {
            println!("{}", head);
            print_list(tail);
        },
        Nil => println!("End of list"),
    }
}





#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}





fn hello(name: &str) {
    println!("Hello, {name}!");
}





struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn print(&self) {
        println!("smart pointer ... {}", self.data);
    }
}   

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(".drop() called by system ... {}", self.data);
    }
}







pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}


