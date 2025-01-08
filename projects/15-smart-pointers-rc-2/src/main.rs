#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    if let Cons(value, next) = &*a {
        println!("Value in a: {}", value.borrow());
        println!("Next in a: {:?}", next);
    }
    if let Cons(value, next) = &b {
        println!("Value in b: {}", value.borrow());
        println!("Next in b: {:?}", next);
    }

    if let Cons(value, next) = &c {
        println!("Value in c: {}", value.borrow());
        println!("Next in c: {:?}", next);
    }
}