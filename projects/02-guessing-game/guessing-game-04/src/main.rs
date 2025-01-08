use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    println!("Guess the number");

    loop {

        let secret_number = rand::thread_rng().gen_range(1..=100);

        let mut guess = String::new();
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess_as_integer: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You guessed: {guess_as_integer} and adding 2 gives {}", guess_as_integer + 2);
    
        match guess_as_integer.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
