use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                    Codewars                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Phone Number                       ====");
    println!("==============================================================");
    println!("... get phone number as string from array of input digits ...");
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let phone_number = create_phone_number(&numbers);
    println!("{}", phone_number);
    let phone_number_2 = create_phone_number_2(&numbers);
    println!("{}", phone_number_2);
    let phone_number_3 = create_phone_number_3(numbers);
    println!("{}", phone_number_3);

    println!("==============================================================");
    println!("==== Find The Digit Which Appears And Odd Number Of Times ====");
    println!("==============================================================");
    println!("... given an array of integers, find the one that appears an odd number of times ...");

    let numbers = [1,2,1];
    let odd_number = find_digit_which_occurs_an_odd_number_of_times(&numbers);

    #[cfg(test)]
    assert_eq!(odd_number, 2);
    println!("{}", odd_number);

    let numbers = [1,2,1,2,3,3,4,4,5,5,5];
    let odd_number = find_digit_which_occurs_an_odd_number_of_times(&numbers);

    #[cfg(test)]
    assert_eq!(odd_number, 5);
    println!("{}", odd_number);

    let numbers = [1,2,1,2,3,3,4,4,5,5,5];
    let odd_number = find_digit_which_occurs_an_odd_number_of_times_2(&numbers);

    #[cfg(test)]
    assert_eq!(odd_number, 5);
    println!("{}", odd_number);

    println!("==============================================================");
    println!("====                  Persistence                         ====");
    println!("==============================================================");

    println!("... given a positive integer, return its multiplicative persistence ...");
    println!("... eg 39 => 3*9 = 27 => 2*7 = 14 => 1*4 = 4 ...");
    let number = 39;
    let persistence = get_persistence(number);
    println!("persistence of number {} is {}", number, persistence);

    let number = 999;
    let persistence = get_persistence(number);
    println!("persistence of number {} is {}", number, persistence);

    let number = 4;
    let persistence = get_persistence(number);
    println!("persistence of number {} is {}", number, persistence);

    println!("==============================================================");
    println!("====                  Printer Errors                      ====");
    println!("==============================================================");

    println!("... given a string of letters, return the number of errors ...");
    println!("... errors are letters not in the range a-m ...");

    let s = "aaabbbbhaijjjm";
    let printer_errors = get_printer_error(s);
    println!("{}", printer_errors);

    let s = "aaaxbbbbyyhwawiwjjjwwm";
    let printer_errors = get_printer_error(s);
    println!("{}", printer_errors);

    println!("==============================================================");
    println!("====         Write Number In Expanded Form                ====");
    println!("==============================================================");

    println!("... given a number, return it in expanded form ...");

    let number = 70304;
    let expanded_form = get_expanded_form(number);
    println!("{}", expanded_form);

    let number = 12;
    let expanded_form = get_expanded_form(number);
    println!("{}", expanded_form);

    let number = 9000000;
    let expanded_form = get_expanded_form(number);
    println!("{}", expanded_form);

    let number = 70304;
    let expanded_form = get_expanded_form_2(number);
    println!("{}", expanded_form);

    println!("==============================================================");
    println!("====                Who Likes Your Post                   ====");
    println!("==============================================================");

    println!("... given a list of names, return a string with the number of likes ...");

    let names = vec!["Peter"];
    let likes_string = get_likes_string(&names);
    println!("{}", likes_string);

    let names = vec!["Jacob", "Alex"];
    let likes_string = get_likes_string(&names);
    println!("{}", likes_string);

    let names = vec!["Max", "John", "Mark"];
    let likes_string = get_likes_string(&names);
    println!("{}", likes_string);

    let names = vec!["Alex", "Jacob", "Mark", "Max"];
    let likes_string = get_likes_string(&names);
    println!("{}", likes_string);

    println!("==============================================================");
    println!("====                    Score The Dice                    ====");
    println!("==============================================================");

    println!("... given five six-sided dice, score the roll ...");
    println!("... 3 x 1 = 1000 points, 3 x 6 = 600 points, 3 x 5 = 500 points, 3 x 4 = 400 points, 3 x 3 = 300 points, 3 x 2 = 200 points ...");

    let dice_rolls = [2, 3, 4, 6, 2];
    let combined_dice_score = get_dice_score(&dice_rolls);
    println!("combined dice score is {}", combined_dice_score);

    let dice_rolls = [4, 4, 4, 3, 3];
    let combined_dice_score = get_dice_score(&dice_rolls);
    println!("combined dice score is {}", combined_dice_score);

    let dice_rolls = [2, 4, 4, 5, 4];
    let combined_dice_score = get_dice_score(&dice_rolls);
    println!("combined dice score is {}", combined_dice_score);

    println!("==============================================================");
    println!("====          Find Pairs Which Add To Given Sum           ====");
    println!("==============================================================");

    println!("... given a list of integers and a sum, find the pairs that add to the sum ...");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = 10;
    let pairs = find_pairs_that_add_to_sum(&numbers, sum);
    println!("pairs that add to sum {} are {:?}", sum, pairs);

    #[cfg(test)]
    assert_eq!(pairs, vec![(1, 9), (2, 8), (3, 7), (4, 6)]);

    println!("==============================================================");
    println!("====         Find First Pair Which Add To Given Sum       ====");
    println!("==============================================================");

    println!("... given a list of integers and a sum, find the first pair that adds to the sum ...");
    println!("... the first pair is determined by the lowest index of the second item in the pair ...");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = 10;
    let pair = find_first_pair_that_adds_to_sum(&numbers, sum);
    println!("first pair that adds to sum {} is {:?}", sum, pair);
    assert_eq!(pair, Some((4, 6)));

    let numbers = [11,3,7,5];
    let sum = 10;
    let pair = find_first_pair_that_adds_to_sum(&numbers, sum);
    println!("first pair that adds to sum {} is {:?}", sum, pair);
    assert_eq!(pair, Some((3, 7)));

    println!("==============================================================");
    println!("====           Simplify Compass Directions                ====");
    println!("==============================================================");

    println!("... given a list of compass directions, simplify the list ...");

    let directions = [Direction::North, Direction::South, Direction::South, Direction::East, Direction::West, Direction::North, Direction::West];
    let simplified_directions = get_simplified_directions(&directions);
    println!("simplified directions are {:?}", simplified_directions);

    println!("==============================================================");
    println!("====                Fibonacci products                   ====");
    println!("==============================================================");

    println!("... given a number, determine if any consecutive fibonacci numbers multiply to give this number ...");
    println!("... eg 714 = 21 * 34 ...");
    println!("... so fn(714) returns (21, 34, true) ...");
    println!("...    fn(800) returns (34, 55, false) ...");

    let number = 714;
    let fibonacci_products = validate_if_number_is_a_fibonacci_product(number);
    println!("fibonacci products for number {} are {:?}", number, fibonacci_products);

    let number = 800;
    let fibonacci_products = validate_if_number_is_a_fibonacci_product(number);
    println!("fibonacci products for number {} are {:?}", number, fibonacci_products);

    println!("==============================================================");
    println!("====                   PIN number                         ====");
    println!("==============================================================");

    println!("... given a list of possible PIN numbers, return all possible PIN numbers ...");
    println!("... maximum PIN length is 8 digits ...");
    println!("... 1 can be 1, 2, 4 ...");
    println!("... 2 can be 1, 2, 3, 5 ...");
    println!("... 3 can be 2, 3, 6 ...");
    println!("... 4 can be 1, 4, 5, 7 ...");
    println!("... 5 can be 2, 4, 5, 6, 8 ...");
    println!("... 6 can be 3, 5, 6, 9 ...");
    println!("... 7 can be 4, 7, 8 ...");
    println!("... 8 can be 5, 7, 8, 9, 0 ...");
    println!("... 9 can be 6, 8, 9 ...");
    println!("... 0 can be 0, 8 ...");

    let observed = "1";
    let possible_pins = get_pins(observed);
    println!("\npossible PINs for observed {} are {:?}\n", observed, possible_pins);

    let observed = "12";
    let possible_pins = get_pins(observed);
    println!("\npossible PINs for observed {} are {:?}\n", observed, possible_pins);

    let observed = "123";
    let possible_pins = get_pins(observed);
    println!("\npossible PINs for observed {} are {:?}\n", observed, possible_pins);

    let observed = "1234";
    let possible_pins = get_pins_official_solution(observed);
    println!("\npossible PINs for observed {} are {:?}\n", observed, possible_pins);

    println!("==============================================================");
    println!("====                   Print Seconds As Date String       ====");
    println!("==============================================================");

    println!("... given a number of seconds, return a string in the format 'days, hours, minutes, seconds' ...");

    let seconds = 0;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 62;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 3662;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 85400;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 86400;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 90061;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 2_890_061;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 15_556_951;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 30_556_951;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 31_556_951;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 31_556_952;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 31_556_953;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 62_556_951;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 128_556_951;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    let seconds = 3_967_241;
    let date_string = get_formatted_date_string_from_seconds(seconds);
    println!("date string for {} seconds is {}", seconds, date_string);

    println!("==============================================================");
    println!("====                 Calculate Zeros In Factorial         ====");
    println!("==============================================================");

    println!("... given a number, return the number of trailing zeros in its factorial ...");

    let number:i64 = 5;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:i64 = 10;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:i64 = 14;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:i64 = 15;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:i64 = 20;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:i64 = 25;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:i64 = 30;
    let trailing_zeros = get_trailing_zeros_in_factorial(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:u64 = 5;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:u64 = 10;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:u64 = 14;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}", number, trailing_zeros);

    let number:u64 = 15;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:u64 = 20;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:u64 = 25;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:u64 = 30;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    let number:u64 = 100000;
    let trailing_zeros = get_trailing_zeros_in_factorial2(number);
    println!("trailing zeros in factorial of {} is {}\n\n", number, trailing_zeros);

    println!("==============================================================");
    println!("====                Josephus Problem                      ====");
    println!("==============================================================");

    println!("... given a number of people and a number to count, return the string of people who are eliminated ...");
    println!("... we eliinate every third person and assume a circle ...");

    let number_of_people = 7;
    let count = 3;
    let eliminated_people = josephus_problem(number_of_people, count);
    println!("eliminated people are {:?}", eliminated_people);

    let number_of_people = 10;
    let count = 3;
    let eliminated_people = josephus_problem(number_of_people, count);
    println!("eliminated people are {:?}", eliminated_people);

    println!("==============================================================");
    println!("====                 Tend To Infinity                     ====");
    println!("==============================================================");

    println!("... given a sequence determine if it tends to infinity or zero ... ");

    println!("... u1 = (1 / 1!) * (1!)");
    println!("... u2 = (1 / 2!) * (1! + 2!)");
    println!("... u3 = (1 / 3!) * (1! + 2! + 3!)");
    println!("... u4 = (1 / 4!) * (1! + 2! + 3! + 4!)");
    println!("... u5 = (1 / 5!) * (1! + 2! + 3! + 4! + 5!)");
    println!("... un = (1 / n!) * (1! + 2! + 3! + ... + n!)");

    let sequence = 1;
    let tends_to = tends_to_infinity_or_zero(sequence);
    println!("sequence {} tends to {}", sequence, tends_to);

    let sequence = 1;
    let tends_to = tends_to_infinity_or_zero(sequence);
    println!("sequence {} tends to {}", sequence, tends_to);

    println!("==============================================================");
    println!("====               Convert Seconds To Time                ====");
    println!("==============================================================");

    println!("... given a number of seconds, return the time in the format 'hours:minutes:seconds' ...");

    let seconds = 0;
    let time = convert_seconds_to_time(seconds);

    println!("time for {} seconds is {}", seconds, time);

    let seconds = 5;
    let time = convert_seconds_to_time(seconds);

    println!("time for {} seconds is {}", seconds, time);

    let seconds = 86399;
    let time = convert_seconds_to_time(seconds);

    println!("time for {} seconds is {}", seconds, time);

}

fn create_phone_number(numbers: &[u8]) -> String {
    let mut phone_number = String::from("(");
    for (index, number) in numbers.iter().enumerate() {
        phone_number.push_str(&number.to_string());
        if index == 2 {
            phone_number.push_str(") ");
        } else if index == 5 {
            phone_number.push_str("-");
        }
    }
    phone_number 
  }
  
fn create_phone_number_2(numbers: &[u8]) -> String {
    let phone_number = format!("({}{}{}) {}{}{}-{}{}{}{}", numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7], numbers[8], numbers[9]);
    phone_number
}

fn create_phone_number_3(numbers: [u8; 10]) -> String {
    format!("({}{}{}) {}{}{}-{}{}{}{}", numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7], numbers[8], numbers[9])
}

fn find_digit_which_occurs_an_odd_number_of_times(numbers: &[i32]) -> i32 {
    let mut count = 0;
    for number in numbers {
        for num in numbers {
            if number == num {
                count += 1;
            }
        }
        if count % 2 != 0 {
            return *number;
        }
    }
    0
}

fn find_digit_which_occurs_an_odd_number_of_times_2(numbers: &[i32]) -> i32 {
    for number in numbers {
        if numbers.iter().filter(|&n| n == number).count() % 2 != 0 {
            return *number;
        }
    }
    0
}

fn get_persistence(n: i32) -> i32 {
    let mut number = n;
    let mut count = 0;
    while number > 9 {
        number = number.to_string().chars().map(|c| c.to_digit(10).unwrap()).product::<u32>() as i32;
        count += 1;
    }
    count
}

fn get_printer_error(s: &str) -> String {
    let errors = s.chars().filter(|&c| c > 'm').count();
    format!("{}/{}", errors, s.len())
}

fn get_expanded_form(number: u64) -> String {
    // i32 >= 0

    println!("number: {}", number);

    let mut expanded_form = String::new();
    let mut factor = 1;
    let number_as_digits = number.to_string().chars().map(|c| c.to_string()).collect::<Vec<String>>();
    let mut all_digits_so_far_are_zero: bool = true;
    for digit in number_as_digits.iter().rev() {
        let digit = digit.parse::<u64>().unwrap();
        println!("digit: {}, factor: {}", digit, factor);

        if digit == 0 {
            factor *= 10;
            continue;
        }
        if factor > 1 && !all_digits_so_far_are_zero {
            expanded_form.insert_str(0, " + ");
        }
        expanded_form.insert_str(0, &format!("{}", digit * factor));
        factor *= 10;
        all_digits_so_far_are_zero = false;
    }
    expanded_form
}

fn get_expanded_form_2(number: u64) -> String {
    let mut expanded_form = String::new();
    let mut factor = 1;
    let number_as_digits = number.to_string().chars().map(|c| c.to_string()).collect::<Vec<String>>();
    for digit in number_as_digits.iter().rev() {
        let digit = digit.parse::<u64>().unwrap();
        if digit > 0 {
            expanded_form.insert_str(0, &format!("{}{} + ", digit, "0".repeat(factor as usize)));
        }
        factor += 1;
    }
    expanded_form.pop();
    expanded_form.pop();
    expanded_form
}

fn get_likes_string(names: &Vec<&str>) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], names.len() - 2),
    }
}

fn get_dice_score(dice: &[i32]) -> i32 {

    println!("input - 5 diced are {:?}", dice);

    let mut score = 0;
    // sort the array
    let mut sorted_dice = dice.to_vec();
    sorted_dice.sort();

    // check for 3 of a kind
    let mut three_of_a_kind = false;
    let mut three_of_a_kind_index = 0;
    for i in 0..sorted_dice.len() {
        if i + 2 < sorted_dice.len() && sorted_dice[i] == sorted_dice[i + 1] && sorted_dice[i] == sorted_dice[i + 2] {
            match sorted_dice[i] {
                1 => score += 1000,
                2 => score += 200,
                3 => score += 300,
                4 => score += 400,
                5 => score += 500,
                6 => score += 600,
                _ => (),
            }
            three_of_a_kind = true;
            three_of_a_kind_index = i;
            break;
        }
    }

    // remove three of a kind from the array if found
    if three_of_a_kind {
        sorted_dice.remove(three_of_a_kind_index);
        sorted_dice.remove(three_of_a_kind_index);
        sorted_dice.remove(three_of_a_kind_index);
    }

    // check for 1s and 5s
    for die in sorted_dice {
        match die {
            1 => score += 100,
            5 => score += 50,
            _ => (),
        }
    }

    println!("score: {}", score);

    score

}

fn find_pairs_that_add_to_sum(numbers: &[i32], sum: i32) -> Vec<(i32, i32)> {
    let mut pairs = Vec::new();
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            if numbers[i] + numbers[j] == sum {
                pairs.push((numbers[i], numbers[j]));
            }
        }
    }
    println!("pairs that add to sum {} are {:?}", sum, pairs);
    pairs
}

fn find_first_pair_that_adds_to_sum(numbers: &[i32], sum: i32) -> Option<(i32, i32)> {

    let mut pair_with_index = (0, 0, 0);
    let mut pair_found = false;
    let mut break_all = false;

    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            if i > j || (pair_found && i > pair_with_index.2){
                break_all = true;
                break;
            }
            if numbers[i] + numbers[j] == sum {
                if pair_with_index.2 == 0 || j < pair_with_index.2 {
                    pair_with_index = (numbers[i], numbers[j], j);
                    pair_found = true;
                }
            }
        }
        if break_all {
            break;
        }
    }
    if pair_with_index.2 == 0 {
        None
    } else {
        Some((pair_with_index.0, pair_with_index.1))
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_simplified_directions(directions: &[Direction]) -> Vec<Direction> {
    let mut simplified_directions = Vec::new();
    for direction in directions {
        let last_direction = simplified_directions.last();
        match direction {
            Direction::North => {
                if last_direction == Some(&Direction::South) {
                    simplified_directions.pop();
                } else {
                    simplified_directions.push(Direction::North);
                }
            }
            Direction::South => {
                if last_direction == Some(&Direction::North) {
                    simplified_directions.pop();
                } else {
                    simplified_directions.push(Direction::South);
                }
            }
            Direction::East => {
                if last_direction == Some(&Direction::West) {
                    simplified_directions.pop();
                } else {
                    simplified_directions.push(Direction::East);
                }
            }
            Direction::West => {
                if last_direction == Some(&Direction::East) {
                    simplified_directions.pop();
                } else {
                    simplified_directions.push(Direction::West);
                }
            }
        }
    }
    simplified_directions
}

fn validate_if_number_is_a_fibonacci_product(number: u64) -> (u64, u64, bool) {

    println!("looking for a product match on number {}", number);
    let mut fibonacci_sequence = vec![0, 1];
    let fib_product_found = false;

    if number == 0 {
        return (0, 0, true);
    }

    fibonacci_sequence.push(1);

    if number == 1 {
        return (1, 1, true);
    }

    while !fib_product_found {
        let last_fib = fibonacci_sequence[fibonacci_sequence.len() - 2];
        let current_fib = fibonacci_sequence[fibonacci_sequence.len() - 1];
        let next_fib = last_fib + current_fib;
        fibonacci_sequence.push(next_fib);

        println!("... fibonacci sequence is {:?}", fibonacci_sequence);

        println!("... product of {} and {} is {}", current_fib, next_fib, current_fib * next_fib);
    
        if current_fib * next_fib == number {
            return (current_fib, next_fib, true);
        } else if current_fib * next_fib > number {
            return (current_fib, next_fib, false);
        }
    }
    (0, 0, false)
}

fn get_pins(observed: &str) -> Vec<String> {

    println!("observed PIN is {}", observed);

    let pin_map: HashMap<char, Vec<char>> = [
        ('1', vec!['1', '2', '4']),
        ('2', vec!['1', '2', '3', '5']),
        ('3', vec!['2', '3', '6']),
        ('4', vec!['1', '4', '5', '7']),
        ('5', vec!['2', '4', '5', '6', '8']),
        ('6', vec!['3', '5', '6', '9']),
        ('7', vec!['4', '7', '8']),
        ('8', vec!['5', '7', '8', '9', '0']),
        ('9', vec!['6', '8', '9']),
        ('0', vec!['0', '8']),
    ].iter().cloned().collect();

    let mut output_vector_length = 0;
    for (index, digit) in observed.chars().enumerate() {
        let possible_digits = pin_map.get(&digit).unwrap();
        if index == 0 {
            output_vector_length = possible_digits.len();
        } else {
            output_vector_length *= possible_digits.len();
        }
    }

    let mut output_vector_brief = vec!["".to_string(); observed.len()];

    println!("output vector length is {}", output_vector_length);

    let mut output_vector = vec!["".to_string(); output_vector_length];
    println!("... now fill the ouptut vector ... ");
    println!("... each string in the output vector will have length {}", observed.len());

    let mut _cumulative_index = 0;

    for (outer_index, digit) in observed.chars().enumerate() {

        output_vector_brief[outer_index] = pin_map.get(&digit).unwrap().iter().map(|c| c.to_string()).collect::<String>();
    }

    println!("\n... output vector brief is {:?}\n", output_vector_brief);

    for (index, possible_pins) in output_vector_brief.into_iter().enumerate() {

        println!("... index: {}, possible pins: {}", index, possible_pins);

    }






    for (outer_index, digit) in observed.chars().enumerate() {

        println!("... input digit: {}", digit);

        let possible_digits = pin_map.get(&digit).unwrap();

        println!("... possible digits are {:?}", possible_digits);
        
        for (inner_index, possible_digit) in possible_digits.into_iter().enumerate() {

            println!("... possible digit: {} at inner index {}", possible_digit, inner_index);

            let combined_index = outer_index + inner_index;
        
            let mut output_vector_clone = output_vector.clone();

            if outer_index == 0 {
                output_vector_clone[combined_index] = possible_digit.to_string();

                println!("... output vector index: {}, output vector value: {}", combined_index, output_vector_clone[combined_index]);

            } else {
                output_vector_clone[combined_index] = output_vector[combined_index - 1].clone() + &possible_digit.to_string();
            }
            
            output_vector = output_vector_clone;
        }

        _cumulative_index += possible_digits.len();
    }

    output_vector
}





fn get_pins_official_solution(observed: &str) -> Vec<String> {
    let possibilities = HashMap::from([
        ('0', vec!['0','8']),
        ('1', vec!['1','2','4']),
        ('2', vec!['1','2','3','5']),
        ('3', vec!['2','3','6']),
        ('4', vec!['1','4','5','7']),
        ('5', vec!['2','4','5','6','8']),
        ('6', vec!['3','5','6','9']),
        ('7', vec!['4','7','8']),
        ('8', vec!['0','5','7','8','9']),
        ('9', vec!['6','8','9']),
    ]);
    
    observed.chars()
        .map(|c| possibilities.get(&c).unwrap())
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect())
        .collect()
}

fn get_formatted_date_string_from_seconds(seconds: i32) -> String {

    if seconds == 0 {
        return "now".to_string();
    }

    let years   = seconds / 31536000;
    let days    = (seconds % 31536000 ) / 86400;
    let hours   = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    if years == 0 && days == 0 && hours == 0 && minutes == 0 {

        if seconds == 1 {
            return format!("{} second", seconds);
        }

        return format!("{} seconds", seconds);
    }

    let mut date_string = String::new();

    if years == 0 && days == 0 && hours == 0 {

        if minutes == 1 {
            date_string.push_str(&format!("1 minute"));
        } else {
            date_string.push_str(&format!("{} minutes", minutes));
        }

        if seconds == 0 {
            return date_string;
        }

        if seconds == 1 {
            date_string.push_str(&format!(" and 1 second"));
        } else {
            date_string.push_str(&format!(" and {} seconds", seconds));
        }

        return date_string;
    } 

    if years == 0 && days == 0 {

        if hours == 1 {
            date_string.push_str(&format!("{} hour", hours));
        } else {
            date_string.push_str(&format!("{} hours", hours));
        }

        if minutes == 0 {
            // do not mention minutes
        } else {
            if minutes == 1 {
                if seconds == 0 {
                    date_string.push_str(&format!(" and 1 minute"));
                } else {
                    date_string.push_str(&format!(", 1 minute"));
                }
            } else {
                if seconds == 0 {
                    date_string.push_str(&format!(" and {} minutes", minutes));
                } else {
                    date_string.push_str(&format!(", {} minutes", minutes));
                }
            }
        }

        if seconds == 0 {
            return date_string;
        }

        if seconds == 1 {
            date_string.push_str(&format!(" and 1 second"));
        } else {
            date_string.push_str(&format!(" and {} seconds", seconds));
        }

        return date_string;
    }


    if years == 0 {

        if days == 1 {
            date_string.push_str(&format!("{} day", days));
        } else {
            date_string.push_str(&format!("{} days", days));
        }

        if hours == 0 {
            // do not mention hours
        } else {
            if hours == 1 {
                if minutes == 0 && seconds == 0 {
                    date_string.push_str(&format!(" and 1 hour"));
                } else {
                    date_string.push_str(&format!(", 1 hour"));
                }
            } else {
                if minutes == 0 && seconds == 0 {
                    date_string.push_str(&format!(" and {} hours", hours));
                } else {
                    date_string.push_str(&format!(", {} hours", hours));
                }
            }
        }

        if minutes == 0 {
            // do not mention minutes
        } else {
            if minutes == 1 {
                if seconds == 0 {
                    date_string.push_str(&format!(" and 1 minute"));
                } else {
                    date_string.push_str(&format!(", 1 minute"));
                }
            } else {
                if seconds == 0 {
                    date_string.push_str(&format!(" and {} minutes", minutes));
                } else {
                    date_string.push_str(&format!(", {} minutes", minutes));
                }
            }
        }

        if seconds == 0 {
            return date_string;
        }

        if seconds == 1 {
            date_string.push_str(&format!(" and 1 second"));
        } else {
            date_string.push_str(&format!(" and {} seconds", seconds));
        }

        return date_string;
    }

    if years == 1 {
        date_string.push_str(&format!("{} year", years));
    } else {
        date_string.push_str(&format!("{} years", years));
    }

    if days == 0 {
        // do not mention days
    } else {
        if days == 1 {
            if hours == 0 {
                date_string.push_str(&format!(" and 1 day"));
            } else {
                date_string.push_str(&format!(", 1 day"));
            }
        } else {
            if hours == 0 {
                date_string.push_str(&format!(" and {} days", days));
            } else {
                date_string.push_str(&format!(", {} days", days));
            }
        }
    }

    if hours == 0 {
        // do not mention hours
    } else {
        if hours == 1 {
            if minutes == 0 {
                date_string.push_str(&format!(" and 1 hour"));
            } else {
                date_string.push_str(&format!(", 1 hour"));
            }
        } else {
            if minutes == 0 {
                date_string.push_str(&format!(" and {} hours", hours));
            } else {
                date_string.push_str(&format!(", {} hours", hours));
            }
        }
    }

    if minutes == 0 {
        // do not mention minutes
    } else {
        if minutes == 1 {
            if seconds == 0 {
                date_string.push_str(&format!(" and 1 minute"));
            } else {
                date_string.push_str(&format!(", 1 minute"));
            }
        } else {
            if seconds == 0 {
                date_string.push_str(&format!(" and {} minutes", minutes));
            } else {
                date_string.push_str(&format!(", {} minutes", minutes));
            }
        }
    }

    if seconds == 0 {
        return date_string;
    }

    if seconds == 1 {
        date_string.push_str(&format!(" and 1 second"));
    } else {
        date_string.push_str(&format!(" and {} seconds", seconds));
    }

    date_string
}

fn get_trailing_zeros_in_factorial2(n: u64) -> u64 {
    let mut count = 0;
    let mut i = 5;
    while n / i > 0 {
        count += n / i;
        i *= 5;
        println!("count: {}, i: {}", count, i);
    }
    count
}

fn get_trailing_zeros_in_factorial(n: i64) -> i64 {

    println!("\nnumber is {}", n);

    //let mut count = 0;
    
    /*
    if n >=5 && n >=2 {  // 5 1
        count += 1;
    }
    if n >= 10 { // 10 2
        count += 1;
    }
    if n >= 15 { // 15 3
        count += 1;
    }
    if n >= 20 { // 20 4
        count += 1;
    }
    if n >= 25 {  // 25 5
        count += 1;
    }
    if n >= 30 { // 30 6
        count += 1;
    }
    */

    let mut count = n / 5;
    if n >= 30 {
        count += 1;
    }
    count
}

fn josephus_problem(n: i32, step: i32) -> String {
    let mut people: Vec<i32> = (1..=n).collect();

    println!("people are {:?} of length {}", people, people.len());

    let mut eliminated_people: Vec<i32> = Vec::new();

    let mut index = 0;

    index = index + step - 1;
    eliminated_people.push(people.remove(index.try_into().unwrap()));
    println!("\npeople are {:?} of length {}", people, people.len());
    println!("eliminated people are {:?} of length {}", eliminated_people, eliminated_people.len());
    
    index = index + step - 1;
    eliminated_people.push(people.remove(index.try_into().unwrap()));
    println!("\npeople are {:?} of length {}", people, people.len());
    println!("eliminated people are {:?} of length {}", eliminated_people, eliminated_people.len());

    index = index + step - 1;

    let people_length = people.len().try_into().unwrap();
    if index >= people_length {
        index = index % people_length;
    }
    let index_to_remove = index.try_into().unwrap();
    eliminated_people.push(people.remove(index_to_remove));
    println!("\npeople are {:?} of length {}", people, people.len());
    println!("eliminated people are {:?} of length {}", eliminated_people, eliminated_people.len());

    index = index + step - 1;

    while people.len() > 0 {
        let people_length = people.len().try_into().unwrap();
        if index >= people_length {
            index = index % people_length;
        }
        let index_to_remove = index.try_into().unwrap();
        eliminated_people.push(people.remove(index_to_remove));
        println!("\npeople are {:?} of length {}", people, people.len());
        println!("eliminated people are {:?} of length {}", eliminated_people, eliminated_people.len());
        index = index + step - 1;
    }


    let mut eliminated_people_string = String::new();
    for (index, person) in eliminated_people.iter().enumerate() {
        if index == 0 {
            eliminated_people_string.push_str(&person.to_string());
        } else {
            eliminated_people_string.push_str(&format!(", {}", person));
        }
    }
    eliminated_people_string
}

fn tends_to_infinity_or_zero(sequence: i32) -> f32 {

    let mut output = 0.0;

    let mut counter = 0;

    counter = 1;
    output = 1.0;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    let integer_32_bit: i32 = 1000;

    let float_32_bit = integer_32_bit as f32;

    println!("The value of float_32_bit is: {}", float_32_bit);

    counter = 2;
    output = 1.0 / 2.0 * (1 + 2) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 3;
    output = 1.0 / 6.0 * (1 + 2 + 6) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 4;
    output = 1.0 / 24.0 * (1 + 2 + 6 + 24) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 5;
    let factorial_5 = 24 * 5;
    let factorial_5_as_f32 = factorial_5 as f32;
    output = 1.0 / factorial_5_as_f32 * (1 + 2 + 6 + 24 + factorial_5) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 6;
    let factorial_6 = factorial_5 * 6;
    let factorial_6_as_f32 = factorial_6 as f32;
    let output = 1.0 / factorial_6_as_f32 * (1 + 2 + 6 + 24 + factorial_5 + factorial_6) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 7;
    let factorial_7 = factorial_6 * 7;
    let factorial_7_as_f32 = factorial_7 as f32;
    let output = 1.0 / factorial_7_as_f32 * (1 + 2 + 6 + 24 + factorial_5 + factorial_6 + factorial_7) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);

    counter = 8;
    let factorial_8 = factorial_7 * 8;
    let factorial_8_as_f32 = factorial_8 as f32;
    let output = 1.0 / factorial_8_as_f32 * (1 + 2 + 6 + 24 + factorial_5 + factorial_6 + factorial_7 + factorial_8) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);
    let output_previous = output;

    counter = 9;
    let factorial_9 = factorial_8 * 9;
    let factorial_9_as_f32 = factorial_9 as f32;
    let output = 1.0 / factorial_9_as_f32 * (1 + 2 + 6 + 24 + factorial_5 + factorial_6 + factorial_7 + factorial_8 + factorial_9) as f32;
    println!("tend to infinity ... count ... {} ... output ... {}", counter, output);
    
    let factorial_instance = Factorial {
        number: 9,
        factorial: factorial_9,
        sum: 1 + 2 + 6 + 24 + factorial_5 + factorial_6 + factorial_7 + factorial_8 + factorial_9,
    };

    let mut factorials = vec![
        Factorial { number: 1, factorial: 1, sum: 1 },
        Factorial { number: 2, factorial: 2, sum: 1 + 2 },
        Factorial { number: 3, factorial: 6, sum: 1 + 2 + 6 },
        Factorial { number: 4, factorial: 24, sum: 1 + 2 + 6 + 24 },
        Factorial { number: 5, factorial: 120, sum: 1 + 2 + 6 + 24 + 120 },
        Factorial { number: 6, factorial: 720, sum: 1 + 2 + 6 + 24 + 120 + 720 },
        Factorial { number: 7, factorial: 5040, sum: 1 + 2 + 6 + 24 + 120 + 720 + 5040 },
        Factorial { number: 8, factorial: 40320, sum: 1 + 2 + 6 + 24 + 120 + 720 + 5040 + 40320 },
        Factorial { number: 9, factorial: 362880, sum: 1 + 2 + 6 + 24 + 120 + 720 + 5040 + 40320 + 362880 },
    ];

    let minimum_delta = 0.01;
    let mut output_delta = output_previous - output;

    println!("output delta is {}", output_delta);
    println!("minimum delta is {}", minimum_delta);

    while output_delta > minimum_delta {
        counter += 1;

        let last_factorial_instance = factorials.last().unwrap();
        let last_sum = last_factorial_instance.sum;

        let factorial = last_factorial_instance.factorial * counter;
        let factorial_as_f32 = factorial as f32;
        let sum = last_sum + factorial;

        let factorial_instance = Factorial {
            number: counter,
            factorial: factorial,
            sum: sum,
        };

        let output_previous = output;
        let output = 1.0 / factorial_instance.factorial as f64 * factorial_instance.sum as f64;
        let output_delta = output_previous - output as f32;

        factorials.push(factorial_instance);
        println!("tend to infinity ... count ... {} ... output ... {}", counter, output);
        println!("output delta is {}", output_delta);

        if output_delta < minimum_delta {
            break;
        }

        if counter > 18 {
            break;
        }
    }
    
    output

}

struct Factorial {
    number: u64,
    factorial: u64,
    sum: u64,
}

fn convert_seconds_to_time(seconds: u32) -> String {

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
