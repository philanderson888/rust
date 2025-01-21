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