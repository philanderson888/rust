fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                     Testing                          ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====                   Introduction                       ====");
    println!("==============================================================");

    println!("testing performs the following tasks:");
    println!("1. Unit Testing");
    println!("2. Integration Testing");
    println!("3. Documentation Testing helps to ensure that the documentation is accurate and up-to-date");
    println!("4. Benchmark Testing");
    println!("5. Property Testing");
    println!("6. Fuzz Testing");
    println!("7. Mutation Testing");
    println!("8. Regression Testing");
    println!("9. Smoke Testing");
    println!("10. Stress Testing");
    println!("11. Usability Testing");
    println!("12. Acceptance Testing");
    println!("13. Alpha Testing");
    println!("14. Beta Testing");
    println!("15. Black Box Testing");
    println!("16. White Box Testing");
    println!("17. Gray Box Testing");
    println!("18. Functional Testing");
    println!("19. Non-Functional Testing");
    println!("20. Manual Testing");
    println!("21. Automated Testing");
    println!("22. Static Testing");
    println!("23. Dynamic Testing");
    println!("24. Code Coverage Testing");
    println!("25. Load Testing");
    println!("26. Performance Testing");
    println!("27. Security Testing");
    println!("28. Penetration Testing");
    println!("29. Vulnerability Testing");
    println!("30. Compliance Testing");
    println!("31. Compatibility Testing");
    println!("32. Conformance Testing");
    println!("33. Exploratory Testing");

    println!("\nwhen we write testing code it does the following:");
    println!("... setup the environment");
    println!("... run the code");
    println!("... check the output");

    println!("\n==============================================================");
    println!("====                   Unit Testing                       ====");
    println!("==============================================================");

    println!("Unit Testing is the process of testing individual units or components of a software. The purpose is to validate that each unit of the software performs as designed. A unit is the smallest testable part of any software. It usually has one or a few inputs and usually a single output. In procedural programming, a unit may be an individual function or procedure. In object-oriented programming, a unit is usually a method or a class.");

    println!("... a rust unit test is a function that's annotated with #[test] attribute");
    println!("... when we use the #[cfg(test)] attribute, the compiler will only compile the test code when we run the tests");
    println!("... the test code is not included in the compiled binary");
    println!("... tests can be used on public and private functions ... ");


    println!("... to run the test, we use the command: cargo test");
    println!("... to run the test with output, we use the command: cargo test -- --nocapture");
    println!("... to run the test with output, we use the command: cargo test -- --show-output");

    println!("... to run the test for a specific function, we use the command: cargo test function_name");
    println!("... to run the test for a specific module, we use the command: cargo test module_name");
    println!("... to run the test for a specific module, we use the command: cargo test module_name::function_name");

    println!("==============================================================");
    println!("====                     Assertions                       ====");
    println!("==============================================================");

    println!("... assert_eq! macro is used to compare two values");
    println!("... assert_ne! macro is used to compare two values");

    println!("==============================================================");
    println!("====                    Should Panic                      ====");
    println!("==============================================================");

    println!("... #[should_panic] attribute is used to test if a function panics");

    println!("==============================================================");
    println!("====                    Custom Messages                   ====");
    println!("==============================================================");

    println!("... custom messages can be added to the assert macro");

    println!("==============================================================");
    println!("====              Using Result in Tests                   ====");
    println!("==============================================================");

    println!("... Result<T, E> can be used in tests to check for errors");

    println!("==============================================================");
    println!("====                 Getting Help                         ====");
    println!("==============================================================");

    println!("cargo test --help");
    println!("cargo test -- --help");

    println!("==============================================================");
    println!("====                     Threads                          ====");
    println!("==============================================================");

    println!("... to run tests in sequence, we use the command: cargo test -- --test-threads=1");

    println!("==============================================================");
    println!("====                 Showing Output                       ====");
    println!("==============================================================");

    println!("... by default passing tests show no output");
    println!("... to show output, we use the command: cargo test -- --show-output");

    println!("==============================================================");
    println!("====            Running A Partial Subset Of Tests         ====");
    println!("==============================================================");

    println!("it is possible to run a partial named subset of tests rather than the full suite ...");

    println!("... firstly we can pass just one test name only as a parameter cargo test function_name");
    println!("... secondly we can do a 'grep' on the tests that we want which have a common part of name ...");
    println!("... eg cargo test add");
    println!("... please note that if a module name is matched, all the tests in that module will be run");
    
    println!("==============================================================");
    println!("====              Ignoring Tests                          ====");
    println!("==============================================================");

    println!("... to ignore a test, we use the #[ignore] attribute");

    println!("... to run ignored tests, we use the command: cargo test -- --ignored");

    println!("... to run tests and ignored tests we use the command cargo test -- --include-ignored");

    println!("==============================================================");
    println!("====                  Integration Tests                   ====");
    println!("==============================================================");

    println!("... unit tests have the tests in same files as the code ... ");
    println!("... integration tests are completely outside our library ...");
    println!("... integration tests call your code as a library and in that sense only have access ...");
    println!("... to any publicly exposed code, not to any private code ...");

    println!("... we create a sibling to 'src' folder and call it 'tests' ... this will hold integration tests ...");

    println!("==============================================================");
    println!("====                  Non-Testing Code                    ====");
    println!("==============================================================");

    let rect1 = Rectangle { width: 8, height: 8 };
    let rect2 = Rectangle { width: 4, height: 4 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));


}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(value: i32) -> i32 {
    value + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_addition() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn addition_with_zero() {
        let result = add(2, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn addition_with_large_numbers() {
        let result = add(1000000000000000000, 1000000000000000000);
        assert_eq!(result, 2000000000000000000);
    }

    #[test]
    fn deliberately_failing_test() {
        let result = add(2, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn deliberately_panic_test() {
        panic!("This test will panic");
    }

    #[test]
    #[ignore]
    fn ignore_this_test(){
        assert_eq!(2, 2);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn can_hold_returns_true() {
        let rect1 = Rectangle { width: 8, height: 8 };
        let rect2 = Rectangle { width: 4, height: 4 };

        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn can_hold_returns_false() {
        let rect1 = Rectangle { width: 4, height: 4 };
        let rect2 = Rectangle { width: 8, height: 8 };

        assert!(!rect1.can_hold(&rect2));
    }
}

#[cfg(test)]
mod addition_tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}



pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod test_greeting {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    #[test]
    fn greeting_contains_name_with_custom_message() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(_value: i32) -> Guess {
        if _value < 1 || _value > 100 {
            panic!("Guess value must be between 1 and 100, got {_value}.");
        }

        Guess { _value }
    }
}

#[cfg(test)]
mod test_guess {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}


#[test]
fn addition_with_result_output() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
