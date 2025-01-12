use rusqlite::{params, Connection, Result};
use rand::Rng;

use faker_rand::en_us::names::{FirstName, LastName};

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
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;

    let random_user_age = rand::thread_rng().gen_range(1..=60);
    println!("random user age is {}", random_user_age);

    /*
    random_name_generator = "0.3.6"
    use rnglib::{RNG, Language};
    let random_name_generator = RNG::try_from(&Language::Roman).unwrap();        
    let first_name = random_name_generator.generate_name();
    println!("{}: {} ", random_name_generator.name, first_name);
    */ 

    let random_first_name = rand::random::<FirstName>().to_string();
    println!("random first name: {}", random_first_name);

    let random_last_name = rand::random::<LastName>().to_string();
    println!("random last name: {}", random_last_name);
    
    conn.execute(
        "INSERT INTO users (first_name, last_name, age) VALUES (?1, ?2, ?3)",
        params![random_first_name, random_last_name, random_user_age],
    )?;

    println!("==============================================================");
    println!("====                     Read Data                        ====");
    println!("==============================================================");

    let mut stmt = conn.prepare("SELECT id, first_name, last_name, age FROM users")?;

    let users: Vec<User> = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            age: row.get(3)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;

    println!("==============================================================");
    println!("====                    Print Data                        ====");
    println!("==============================================================");
    
    for user in &users {
        //println!("{:?}", user);    
        println!("{} - - {} {} is {} years old", user.id, user.first_name, user.last_name, user.age);
    }

    println!("\n\n");

    /*
    for user in &users { 
        println!("second iteration - user {} {} is {} years old and has id {}", user.first_name, user.last_name, user.age, user.id);
    }
    */

    Ok(())
}

#[derive(Debug, Clone)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    age: i32,
}
