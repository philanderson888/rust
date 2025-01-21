use iced::widget::{button, column, text, Column, row, Row};
use iced::{Alignment, Length};
use iced::{Color};
use rusqlite::{params, Connection, Result};

use iced::widget::{Text};
use iced::{Element, Settings};
use iced::font::Weight;
use iced::{color};
use iced::widget::container;
use iced::widget::Container;
use iced::{Background};

use crate::container::Style;

fn main() -> iced::Result {
    println!("Starting application...");

    let conn = initialize_database().expect("Failed to initialize database");

    let initial_value = get_counter_value(&conn).unwrap_or(0);
    let database_counter = Counter::new(initial_value);
    println!("Initial counter value: {:?}", database_counter.value);

    iced::run("A cool counter", Counter::update, Counter::view)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
    Double,
    ExitApp,
}

#[test]
fn it_counts_properly() {
    let mut counter = Counter { value: 0 };

    counter.update(Message::Increment);
    counter.update(Message::Increment);
    counter.update(Message::Decrement);

    assert_eq!(counter.value, 1);
}

#[derive(Debug, Clone, Default)]
struct Counter {
    value: i64,
}

impl Counter {
    fn new(initial_counter_value: i64) -> Self {
        Self { value: initial_counter_value }
    }

    pub fn view(&self) -> Column<Message> {
        let conn = initialize_database().expect("Failed to initialize database");
        let initial_counter_value = get_counter_value(&conn).unwrap_or(0);
        let text_size_1 = 50;
        let box_size = 50;

        let button_widget = button("2/12").width(Length::FillPortion(2));

        let container_widget_01 = container("container with full width and height 50").padding(10).center(box_size)
            .width(Length::Fill)
            .height(50.0);

        let container_widget_02 = container("container half width").padding(10).center(box_size)
            .width(Length::FillPortion(2))
            .height(50.0);

        let container_widget_03 = container("container half width").padding(10).center(box_size)
            .width(Length::Fill)
            .height(50.0);

        let row_05 = iced::widget::row![
                iced::widget::button("-").on_press(Message::Decrement),
                iced::widget::text(self.value),
                iced::widget::button("+").on_press(Message::Increment)
            ];

        let stuff_centered = iced::widget::Container::new(iced::widget::text("Some Text"))
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center)
            .width(Length::Fill)
            .height(Length::Fixed(50.0));

        column![

            row![
                iced::Element::new(button_widget).explain(iced::Color::BLACK),
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
                container_widget_01,
            ],

            row![
                container_widget_02,
                container_widget_03,
            ],

            row![
                button("1").width(Length::Fixed(50.0)),
                button("1").width(Length::Fixed(50.0)),
                button("1").width(Length::Fixed(50.0)),
                button("1").width(Length::Fixed(50.0)),
                button("1").width(Length::Fixed(50.0)),
                button("1").width(Length::Fixed(50.0)),            
            ]
            .spacing(10),


            row_05,

            row![
                stuff_centered
            ],

            row![
                container("1").padding(10).center(box_size)
                .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("2").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("3").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(10),

            row![
                container("1").padding(10).center(box_size)
                .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("2").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("3").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("4").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

            ]
            .spacing(10),

            row![
                container("1").padding(10).center(box_size)
                .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("2").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),

                container("3").padding(10).center(box_size)
                    .style(container::bordered_box)
                    .width(Length::FillPortion(1)),
            ]
            .spacing(10),

        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
    }

    fn update(&mut self, message: Message) {
        let conn = initialize_database().expect("Failed to initialize database");
        let counter_value = get_counter_value(&conn).unwrap_or(0);
        println!("Function update starts with database counter value: {:?}", counter_value);

        match message {
            Message::Increment => {
                println!("Incrementing counter value");
                save_counter_value(&conn, counter_value + 1).expect("Failed to save counter value");
                self.value += 1;
            }
            Message::Decrement => {
                println!("Decrementing counter value");
                save_counter_value(&conn, counter_value - 1).expect("Failed to save counter value");
                self.value -= 1;
            }
            Message::Reset => {
                println!("Resetting counter value");
                save_counter_value(&conn, 0).expect("Failed to save counter value");
                self.value = 0;
            }
            Message::Double => {
                println!("Doubling counter value");
                save_counter_value(&conn, counter_value * 2).expect("Failed to save counter value");
                self.value = counter_value * 2;
            }
            Message::ExitApp => {
                println!("Exiting app");
                std::process::exit(0);
            }
        }
    }
}

fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("counter.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS counter (
            id INTEGER PRIMARY KEY,
            value INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

fn get_counter_value(conn: &Connection) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT value FROM counter WHERE id = 1")?;
    let result = stmt.query_row([], |row| row.get(0));
    Ok(result.unwrap_or(0)) // Default to 0 if no value exists
}

fn save_counter_value(conn: &Connection, value: i64) -> Result<()> {
    println!("Saving counter value: {:?}", value);
    conn.execute(
        "INSERT INTO counter (id, value) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET value = excluded.value",
        params![value],
    )?;
    Ok(())
}
