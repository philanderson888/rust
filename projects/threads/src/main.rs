use std::thread;
use std::time::Duration; 
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                      Threads                         ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Rust provides a powerful concurrency model, but it has a learning curve");
    println!("Threads are lightweight and you can create many of them");
    println!("... concurrent programming allows different parts of your program to execute independently");
    println!("... parallel programming is a subset of concurrent programming ... ");

    println!("\n... rust team have coined a phrase 'fearless concurrency' ... ");
    println!("... compile time errors will be caught and fixed early, increasing reliability on multithreaded programs ... ");

    println!("\ntopics covered ...");
    println!("... creating threads ... ");
    println!("... waiting for all threads to finish ... ");
    println!("... using message passing to transfer data between threads ... ");
    println!("... shared state concurrency ... ");
    println!("... using Mutex<T> to allow access to data from one thread at a time ... ");
    println!("... using Arc<T> to allow multiple threads to access data ... ");
    println!("... using atomic reference counting to allow multiple threads to access data ... ");
    println!("... using channels to transfer data between threads ... ");
    println!("... 'sync' and 'send' traits on user defined types ... ");

    println!("\nproblems with concurrency ...");
    println!("... race conditions ... where data is accessed out of sync ... ");
    println!("... deadlocks ... where two threads are waiting for each other to finish ... ");
    println!("... problems with resource management ... ");
    println!("... problems with performance ... ");
    println!("... obscure bugs ... which are hard to reproduce ... and fix ... ");

    println!("\nrust implements a 1-1 model with the operating system - one thread per operating system thread ... ");
    println!("... this is different from the M:N model where multiple threads are mapped to a smaller number of operating system threads ... ");
    println!("... the 1-1 model is simpler and more predictable ... ");
    println!("... the 1-1 model is more efficient ... ");
    println!("... the 1-1 model is more scalable ... ");
    println!("... the 1-1 model is more portable ... ");
    println!("... the 1-1 model is more reliable ... ");
    println!("... the 1-1 model is more debuggable ... ");
    println!("... crates like 'rayon' and 'crossbeam' provide more advanced threading models ... ");

    println!("\n==============================================================");
    println!("====                   Creating Threads                   ====");
    println!("==============================================================");

    println!("... rust provides a 'thread' module to create threads ... ");
    println!("... the 'thread' module provides a 'spawn' function to create a new thread ... ");
    println!("... the 'spawn' function takes a closure as an argument ... ");
    println!("... the closure takes no arguments and returns nothing ... ");
    println!("... the 'spawn' function returns a 'JoinHandle' ... ");
    println!("... the 'JoinHandle' is a type that allows you to wait for the thread to finish ... ");
    println!("... the 'JoinHandle' is a type that allows you to get the result of the thread ... ");
    println!("... the 'JoinHandle' type is generic ... \n\n");

    thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        println!("number {i} from the main thread!");
        thread::sleep(Duration::from_millis(5));
    }

    println!("\n... once the main thread finishes, the spawned thread will also finish ... ");

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====                   Waiting for Threads                 ====");
    println!("==============================================================");

    println!("... the 'JoinHandle' type has a 'join' method ... ");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..5 {
        println!("number {i} from the main thread!");
        thread::sleep(Duration::from_millis(5));
    }

    handle.join().unwrap();

    println!("\n... the 'join' method will block the main thread until the spawned thread finishes ... ");

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====                   Waiting for Threads 2               ====");
    println!("==============================================================");

    println!("... the 'JoinHandle' type has a 'join' method ... ");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(2));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("number {i} from the main thread!");
        thread::sleep(Duration::from_millis(5));
    }

    println!("\n==============================================================");
    println!("====                 Moving A Thread Handle                ====");
    println!("===============================================================");

    println!("... you can move a thread handle to another thread ... ");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(2));
        }
    });

    let handle2 = handle;

    handle2.join().unwrap();

    for i in 1..10 {
        println!("number {i} from the main thread!");
        thread::sleep(Duration::from_millis(5));
    }

    println!("\nin this example we moved the handle to another thread running on the main ui thread ... ");

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====                 Using Message Passing                 ====");
    println!("==============================================================");

    println!("... you can use message passing to transfer data between threads ... ");

    println!("... the 'std::sync::mpsc' module provides a 'channel' function ... ");
    println!("... mpsc multiple producer single consumer ... ");
    println!("... returns a 'tuple' ... one part to send, other to receive ... ");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello from the spawned thread!");
        println!("... note that val is moved into the message ... ");
        tx.send(val).unwrap();
        println!("... val is not available here ... ");
    });

    println!("... the 'send' method will block the main thread until the spawned thread finishes ... ");
    println!("... the 'recv' method will block the main thread until the spawned thread sends a message ... ");
    println!("... try_recv is the unblocking alternative ... ");
    let received = rx.recv().unwrap();

    println!("Got: {received}");


    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====                 Sending Multiple Messages              ====");
    println!("===============================================================");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====               Sending From Multiple Threads            ====");
    println!("==============================================================");

    println!("... in this instance we spawn multiple threads ... ");
    println!("... each thread sends a message to the main thread ... ");

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====               Shared State Concurrency                 ====");
    println!("==============================================================");

    println!("... shared state concurrency is where multiple threads access the same data ... ");
    println!("... shared memory is like having multiple owners of the same data ... ");
    println!("... shared memory is like having multiple mutable references to the same data ... ");
    println!("... this can cause extreme problemes ... ");

    println!("\nwe use mutexes to allow only one thread to access the data at a time ... ");
    println!("... the 'std::sync::Mutex' type provides a way to lock the data ... ");
    println!("... the 'std::sync::Mutex' type provides a way to unlock the data ... ");
    println!("... the 'std::sync::Mutex' type provides a way to block the thread until the data is unlocked ... ");
    println!("... the 'std::sync::Mutex' type provides a way to return a 'Result' ... ");
    println!("... the 'std::sync::Mutex' type provides a way to return an 'Err' if the data is already locked ... ");
    println!("... the 'std::sync::Mutex' type provides a way to return an 'Ok' if the data is unlocked ... ");
    println!("... the word mutex is short for mutual exclusion ... and indicates only one thread can access the data at a time ... ");

    println!("\nto use the mutex we have to remember ... ");
    println!("... to lock the data before accessing it ... ");
    println!("... to unlock the data after accessing it ... ");

    println!("\n==============================================================");
    println!("====               Using Mutexes                           ====");
    println!("==============================================================");

    let shared_mutable_data = Mutex::new(5);

    {
        println!("... call to lock() returns a 'MutexGuard' ... which implements 'Deref' ... so we can access the value ... ");
        println!("... when we call 'Deref' then also 'Drop' is called and the lock is released ... ");
        let mut locked_instance = shared_mutable_data.lock().unwrap();
        *locked_instance = 6;
    }

    println!("shared mutable data = {shared_mutable_data:?}");
    // Lock the Mutex again to read the value
    let locked_instance = shared_mutable_data.lock().unwrap();
    println!("locked_instance = {:?}", *locked_instance);


    thread::sleep(Duration::from_millis(2000));


    println!("\n==============================================================");
    println!("====               Sharing Mutexes                          ====");
    println!("==============================================================");

    println!("... you can share a mutex between threads ... ");
    println!("... in this case we have one shared counter but ten threads all act upon it and add one to it ... ");
    println!("... every time we act on the data we push the thread handle to a vector ... ");

    let mutable_shared_counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let mutable_shared_counter = Arc::clone(&mutable_shared_counter);
        let handle = thread::spawn(move || {
            let mut num = mutable_shared_counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    println!("... by using handle.join() we are waiting for all threads to finish ... and thus the maximum value of shared counter will be reached ... ");
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *mutable_shared_counter.lock().unwrap());

    thread::sleep(Duration::from_millis(2000));

    println!("\n==============================================================");
    println!("====                         Send                         ====");
    println!("==============================================================");

    println!("... the 'Send' trait is implemented for types that can be sent between threads ... ");
    println!("... it is an indicator of ownership transfer ... when we send we also transfer ownership ... ");

    println!("\n==============================================================");
    println!("====                         Sync                         ====");
    println!("==============================================================");

    println!("... the 'Sync' trait is implemented for types that can be shared between threads ... ");
    println!("... it is an indicator of shared ownership ... when we share we do not transfer ownership ... ");
    println!("... the 'Sync' trait is implemented for types that are thread safe ... ");
    println!("... the 'sync' flag is a shared reference ... ");

    println!("... neither 'send' nor 'sync' should be implemented manually ... they are implemented by the compiler ... ");

    
}
