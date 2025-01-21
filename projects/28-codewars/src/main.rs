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
