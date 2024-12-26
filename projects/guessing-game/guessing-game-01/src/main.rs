use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess_as_integer = guess.trim().parse::<i32>().unwrap();

    println!("You guessed: {guess} and adding 2 gives {}", guess_as_integer + 2);

}
