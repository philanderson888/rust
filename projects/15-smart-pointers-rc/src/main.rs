use List::{Cons, Nil};
use std::rc::Rc;

fn main() {

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

    
    let reference_counted_list = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let list01 = Cons(3, Rc::clone(&reference_counted_list));
    let list02 = Cons(4, Rc::clone(&reference_counted_list));

    println!("... reference counted list = {:?}", reference_counted_list);
    println!("... list 01 = {:?}", list01);
    println!("... list 02 = {:?}", list02);

    println!("... reference counted list has {} strong references", Rc::strong_count(&reference_counted_list));
    
    {
        let list03 = Cons(6, Rc::clone(&reference_counted_list));
        println!("... list 03 = {:?}", list03);
        println!("... reference counted list has {} strong references", Rc::strong_count(&reference_counted_list));
    }

    println!("... reference counted list has {} strong references", Rc::strong_count(&reference_counted_list));

    if let Cons(value, next) = &list01 {
        println!("Value in list 01: {}", value);
        println!("Next in list 01: {:?}", next);
    }

    if let Cons(value, next) = &list02 {
        println!("Value in list 02: {}", value);
        println!("Next in list 02: {:?}", next);
    }

}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
