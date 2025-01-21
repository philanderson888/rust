fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                       Iced                           ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("we will be looking at building rust GUI applications using Iced");

    println!("==============================================================");
    println!("==============================================================");
    println!("====                  Getting Started                     ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("Iced is a cross-platform GUI library for Rust focused on simplicity and type-safety");
    println!("It is heavily inspired by The Elm Architecture and the Elm language");
    println!("Iced is built on top of wgpu, a modern and portable GPU API");
    println!("Iced is designed to be easy to use, with a simple and expressive API");
    println!("Iced is built with performance in mind, and is designed to be fast and efficient");
    println!("Iced has a high-level, easy-to-use API that allows you to build rich user interfaces with minimal boilerplate.");

    println!(" state is represented here by a struct counter ...");

    println!("... messaging is represented by an enum Message ...");

    println!("... the update logic is implemented in the Counter struct ...");

    println!("==============================================================");
    println!("====                   Initialise                        ====");
    println!("==============================================================");

    let mut counter = Counter { value: 0 };
    let mut _counter_clone = counter.clone();
    let mut _counter02 = Counter::new();

    println!("initial counter value: {:?}", counter);

    println!("==============================================================");
    println!("====                    Interact                         ====");
    println!("==============================================================");

    println!("... simulate user interaction by using the messages to update the state ...");

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);

}

// represents state of the counter
#[derive(Debug, Clone)]
struct Counter {
    value: i64,
}

// methods for the counter represent the update logic
impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
}

// represents messages that can be sent to the counter
enum Message {
    Increment,
    Decrement,
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}