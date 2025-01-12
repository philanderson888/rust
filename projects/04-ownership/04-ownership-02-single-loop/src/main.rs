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

    // Query the data
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let users = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for user in users {
        println!("... user? checks if result is OK in which case it returns user object, or an error ... ");
        let user = user?;
        println!("{:?}", user);
        println!("user {} is {} years old and has id {}", user.name, user.age, user.id);
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
    age: i32,
}
