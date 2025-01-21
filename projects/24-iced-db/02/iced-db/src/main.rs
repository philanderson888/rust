use iced::widget::{button, column, text, Column};
use rusqlite::{params, Connection, Result};

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
    println!("====              Initialise Database                     ====");
    println!("==============================================================");

    let conn = initialize_database().expect("Failed to initialize database");

    println!("==============================================================");
    println!("====                   Initialise                        ====");
    println!("==============================================================");

    let initial_value = get_counter_value(&conn).unwrap_or(0);
    let mut database_counter = Counter::new(initial_value);
    println!("initial counter value: {:?}", database_counter.value);

    println!("==============================================================");
    println!("====                    Interact                         ====");
    println!("==============================================================");
    println!("... simulate user interaction by using the messages to update the state ...");
    println!("database counter value: {:?}", database_counter.value);

    println!("==============================================================");
    println!("====                 Save To Database                     ====");
    println!("==============================================================");

    save_counter_value(&conn, database_counter.value).expect("Failed to save counter value");

    println!("==============================================================");
    println!("====                      Test                            ====");
    println!("==============================================================");

    println!("... test the counter value ... run `cargo test` also ...");

    println!("==============================================================");
    println!("====                      GUI                             ====");
    println!("==============================================================");

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
    fn new(initial_counter_value: i64) -> Self {
        Self { value: initial_counter_value  }
    }

    pub fn view(&self) -> Column<Message> {

        let conn = initialize_database().expect("Failed to initialize database");
        let initial_counter_value = get_counter_value(&conn).unwrap_or(0);

        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press({
                println!("button increment handler activated");
                //save_counter_value(&conn, initial_counter_value + 1).expect("Failed to save counter value");
                Message::Increment
            }),

            text(initial_counter_value).size(50),

            /*
            button("-").on_press({
                println!("button decrement handler activated");
                Message::Decrement
            }),
            */
        ]
    }

    fn update(&mut self, message: Message) {

        let conn = initialize_database().expect("Failed to initialize database");
        let counter_value = get_counter_value(&conn).unwrap_or(0);
        println!("function update starts with database counter value reading {:?}", counter_value);

        match message {
            Message::Increment => {
                println!("incrementing counter value");
                save_counter_value(&conn, counter_value + 1).expect("Failed to save counter value");
                self.value += 1
            },
            Message::Decrement => {
                self.value -= 1
            },
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
    println!("saving counter value: {:?}", value);
    conn.execute(
        "INSERT INTO counter (id, value) VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET value = excluded.value",
        params![value],
    )?;
    Ok(())
}