use rusqlite::{params, Connection, Result};

fn main() -> Result<()> {
    // Connect to SQLite database (creates a file if it doesn't exist)
    let conn = Connection::open("local.db")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;

    // Insert a user
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;


    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;

    let users: Vec<User> = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;


    for user in &users {
        println!("{:?}", user);    
        println!("first iteration - user {} is {} years old and has id {}", user.name, user.age, user.id);
    }

    for user in &users {
        println!("{:?}", user);    
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
