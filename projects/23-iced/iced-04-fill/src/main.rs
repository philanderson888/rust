use iced::widget::{button, column, text, Column, row, Row};
use iced::{Alignment, Element, Length, Settings};

fn main() -> iced::Result {
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

    println!("==============================================================");
    println!("====                      Test                            ====");
    println!("==============================================================");

    println!("... test the counter value ... run `cargo test` also ...");

    println!("==============================================================");
    println!("====                      GUI                             ====");
    println!("==============================================================");

    //let increment = button("+");
    //let decrement = button("-");

    //let counter = text(15);

    iced::run("A cool counter", Counter::update, Counter::view)
}



// represents messages that can be sent to the counter
#[derive(Debug, Clone, Copy)]
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


// represents state of the counter
#[derive(Debug, Clone, Default)]
struct Counter {
    value: i64,
}

// methods for the counter represent the update logic
impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    pub fn view(&self) -> Column<Message> {
        column![
            button("Shrink").width(Length::Shrink),
            button("Fill").width(Length::Fill),
            row![
                button("FillPortion 2").width(Length::FillPortion(2)),
                button("FillPortion 1").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            button("Fixed").width(Length::Fixed(100.)),
            button("Fill (height)").height(Length::Fill),
        ]
        .spacing(10)
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }


}