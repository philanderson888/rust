use rusqlite::{params, Connection, Result};
use rand::Rng;

fn main() -> Result<()> {

    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Ownership                       ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                      Database                       ====");
    println!("==============================================================");

    let conn = Connection::open("local.db")?;

    /*
    conn.execute(
        "DROP TABLE IF EXISTS users",
        [],
    )?;
    */

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;

    let random_user_age = rand::thread_rng().gen_range(1..=60);
    println!("random user age is {}", random_user_age);

    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Alice", random_user_age],
    )?;

    println!("==============================================================");
    println!("====                     Read Data                        ====");
    println!("==============================================================");

    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;

    let users: Vec<User> = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    println!("==============================================================");
    println!("====                    Print Data                        ====");
    println!("==============================================================");
    
    for user in &users {
        println!("{:?}", user);    
        println!("first iteration - user {} is {} years old and has id {}", user.name, user.age, user.id);
    }

    println!("\n\n");

    for user in &users { 
        println!("second iteration - user {} is {} years old and has id {}", user.name, user.age, user.id);
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    age: i32,
}
