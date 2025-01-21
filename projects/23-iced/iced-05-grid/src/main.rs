use iced::widget::{button, column, text, Column, row, Row, Text};
use iced::{Alignment, Element, Length, Settings,};
use iced::font::Weight;
use iced::{color};
use iced::widget::container;
use iced::widget::Container;
use iced::{Background, Color};

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


    println!("==============================================================");
    println!("====                    Reference                        ====");
    println!("==============================================================");
    println!("... examples ... https://github.com/iced-rs/iced/tree/master");

    println!("\n==============================================================");
    println!("====                    Example                          ====");
    println!("==============================================================");

    println!(" state is represented here by a struct counter ...");

    println!("... messaging is represented by an enum Message ...");

    println!("... the update logic is implemented in the Counter struct ...");




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

    iced::run("Rust Iced Grid Rows and Columns", Counter::update, Counter::view)
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
            row![
                button("1").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/2").width(Length::FillPortion(1)),
                button("1/2").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/3").width(Length::FillPortion(1)),
                button("1/3").width(Length::FillPortion(1)),
                button("1/3").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/4").width(Length::FillPortion(1)),
                button("1/4").width(Length::FillPortion(1)),
                button("1/4").width(Length::FillPortion(1)),
                button("1/4").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/5").width(Length::FillPortion(1)),
                button("1/5").width(Length::FillPortion(1)),
                button("1/5").width(Length::FillPortion(1)),
                button("1/5").width(Length::FillPortion(1)),
                button("1/5").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/6").width(Length::FillPortion(1)),
                button("1/6").width(Length::FillPortion(1)),
                button("1/6").width(Length::FillPortion(1)),
                button("1/6").width(Length::FillPortion(1)),
                button("1/6").width(Length::FillPortion(1)),
                button("1/6").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
            ]
            .spacing(10),
            row![
                button("Cell").width(Length::FillPortion(2)),
                button("Cell").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                text("some text").center().width(Length::FillPortion(3)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                text("right text").align_x(Alignment::End).width(Length::FillPortion(3)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                text("left text").align_x(Alignment::Start).color(color!(0x0000ff)).width(Length::FillPortion(3)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                text("blue text").color(color!(0x0000ff)).center().width(Length::FillPortion(3)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                text("text size 20").size(20).color(color!(0x0000ff)).center().width(Length::FillPortion(3)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
                button("2/12").width(Length::FillPortion(2)),
                button("1/12").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![
                container("Container").padding(10).center(300)
                    .style(container::rounded_box)
                    .width(Length::FillPortion(1)),
                button("1/12").width(Length::FillPortion(5)),
            ]
            .spacing(10),
            row![
                container("Container").padding(10).center(300)
                    .style(container::rounded_box)
                    .width(Length::FillPortion(1)),
                button("1/12").width(Length::FillPortion(5)),
            ]
            .spacing(10),
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
