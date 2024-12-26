fn main() {
    println!("==============================================================");
    println!("==============================================================");
    println!("====                 Structs And Enums                    ====");
    println!("==============================================================");
    println!("==============================================================");

    println!("==============================================================");
    println!("====           Defining and Instantiating Structs         ====");
    println!("==============================================================");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1: email: {}, username: {}, active: {}, sign_in_count: {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let mut user2 = User {
        email: String::from("someone2@example.com"),
        username: String::from("someusername1234"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("alteredemail@example.com");

    println!("User2: email: {}, username: {}, active: {}, sign_in_count: {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let user3 = build_user(String::from("someone3@example.com"), String::from("someusername12345"));

    println!("User3: email: {}, username: {}, active: {}, sign_in_count: {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    println!("... we can even copy details from another struct");

    println!("... using struct update syntax");

    let user4 = User {
        email: String::from("someone4@example.com"),
        username: String::from("someusername123456"),
        ..user3 
    };

    println!("User4: email: {}, username: {}, active: {}, sign_in_count: {}", user4.email, user4.username, user4.active, user4.sign_in_count);

    println!("==============================================================");
    println!("====           Tuple Structs                               ====");
    println!("==============================================================");

    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let green = Color(0, 255, 0);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("White: {}, {}, {}", white.0, white.1, white.2);
    println!("Green: {}, {}, {}", green.0, green.1, green.2);

    struct Point(i32, i32, i32);

    let origin = Point(0, 0, 0);

    println!("Origin: {}, {}, {}", origin.0, origin.1, origin.2);

    println!("... note that you cannot access the fields by name but only by index ... 0, 1, 2 in this case");

    println!("==============================================================");
    println!("====           Unit-Like Structs                           ====");
    println!("==============================================================");

    struct UnitLikeStruct;

    let _unit_like_struct = UnitLikeStruct;

    println!("... this is a unit-like struct");

    println!("... these can be useful for testing purposes when you don't need any data to be associated with the struct");

    println!("==============================================================");
    println!("====                 Rectangle Example                    ====");
    println!("==============================================================");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle 1: width: {}, height: {}", rect1.width, rect1.height);

    println!("... the area of the rectangle is: {}", get_area(rect1));

    println!("... we can also use references to the struct");

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    println!("... the area of the rectangle is: {}", get_area_ref(&rect2));

    println!("... at this point in the code, rect2 is still available for use because we only passed its reference.  rect1 is not available because it was moved into the get_area function");

    println!("... to print our structs, we can use the Debug trait by adding #[derive(Debug)] to the struct definition");

    println!("Rectangle 2: {:?}", rect2);
    println!("Rectangle 1: {:#?}", rect2);

    println!("==============================================================");
    println!("====                 Methods                               ====");
    println!("==============================================================");

    println!("... methods are defined within the context of a struct");

    println!("... we use the impl (implementation) keyword to define methods for a struct");
    println!("... the first parameter of a method is always `self`");
    println!("... note that the `impl` keyword can be added to separate blocks");

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Rectangle 3: {:?} ", rect3);
    println!("... the area of the rectangle is: {}", rect3.area());
    println!("... and rect3 is still available for use because we only passed a reference to the area method");
    println!("Rectangle 3: {:?} ", rect3);

    if rect3.has_width() {
        println!("... the rectangle has a width of {}", rect3.width);
    } else {
        println!("... the rectangle does not have a width");
    }

    println!("... as for regular objects, you can pass a reference `&self` or a mutable reference `&mut self` or move ownership using `self`");

    println!("==============================================================");
    println!("====           Methods With More Parameters               ====");
    println!("==============================================================");

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect5 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect6 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Rectangle 4: {:?} ", rect4);
    println!("Rectangle 5: {:?} ", rect5);
    println!("Rectangle 6: {:?} ", rect6);

    println!("... can rect4 hold rect5? {}", rect4.can_hold(&rect5));
    println!("... can rect4 hold rect6? {}", rect4.can_hold(&rect6));

    println!("==============================================================");
    println!("====                  Constructors                        ====");
    println!("==============================================================");

    println!("... we can add a constructor to a struct ... ");

    println!("==============================================================");
    println!("====           Associated Functions                       ====");
    println!("==============================================================");

    println!("... associated functions are defined within the context of a struct but do not take a reference to the struct as a parameter");

    println!("... they are often used for constructors");

    let square = Rectangle::square(10);

    println!("Square: {:?}", square);

    println!("==============================================================");
    println!("====                  Constructors                        ====");
    println!("==============================================================");

    println!("... we can add a constructor to a struct ... ");

    let my_struct_instance = MyStruct::new(10, 20);

    println!("My Struct Instance: {:?}", my_struct_instance);
    println!("... the fields of the struct are: {}, {}", my_struct_instance.field01, my_struct_instance.field02);
    println!("... the result of the method is: {}", my_struct_instance.my_method());

    println!("==============================================================");
    println!("====                     Enums                            ====");
    println!("==============================================================");

    println!("... enums are a way to define a type by enumerating its possible values");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("V4: {:?}", four);
    println!("V6: {:?}", six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("10.20.30.40"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    println!("... we can also explicitly read the fields ... ");

    println!("home.kind: {:?}", home.kind);
    println!("home.address: {}", home.address);
    println!("loopback.kind: {:?}", loopback.kind);
    println!("loopback.address: {}", loopback.address);

    if let IpAddrKind::V4 = home.kind {
        println!("Home IP: {}", home.address);
    }
    if let IpAddrKind::V6 = loopback.kind {
        println!("Loopback IP: {}", loopback.address);
    }

    println!("\n\n... we can also use the 'route' function to take an enum as a parameter\n\n");

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    println!("\n\n... in this case we can see that we have created a separate enum and struct for the IP address");
    println!("... we can actually combine the two into a single enum\n");

    let home = IpAddrEnum::VV4(String::from("10.20.30.40"));
    let loopback = IpAddrEnum::VV6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);
    println!("... explicitly reading the fields ... ");

    if let IpAddrEnum::VV4(ip) = home {
        println!("Home IP: {}", ip);
    }
    if let IpAddrEnum::VV6(ip) = loopback {
        println!("Loopback IP: {}", ip);
    }

    println!("\n... we can also use tuples to define the enum\n\n");

    let home = IpAddr2Enum::VVV4(127, 0, 0, 1);
    let loopback = IpAddr2Enum::VVV6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    if let IpAddr2Enum::VVV4(a, b, c, d) = home {
        println!("Home IP: {}.{}.{}.{}", a, b, c, d);
    }

    if let IpAddr2Enum::VVV6(ip) = loopback {
        println!("Loopback IP: {}", ip);
    }

    println!("==============================================================");
    println!("====             Enum With Different Types                ====");
    println!("==============================================================");

    println!("... we can also define enums with different types");

    let message01 = Message::Write(String::from("hello"));

    let message02 = Message::Move { x: 10, y: 20 };

    let message03 = Message::ChangeColor(0, 255, 0);

    let message04 = Message::Quit;

    println!("Message01: {:?}", message01);
    println!("Message02: {:?}", message02);
    println!("Message03: {:?}", message03);
    println!("Message04: {:?}", message04);

    println!("... we can also explicitly access the fields ...");

    if let Message::Write(message) = message01 {
        println!("Message01: {}", message);
    }

    if let Message::Move { x, y } = message02 {
        println!("Message02: x: {}, y: {}", x, y);
    }

    if let Message::ChangeColor(r, g, b) = message03 {
        println!("Message03: r: {}, g: {}, b: {}", r, g, b);
    }

    println!("==============================================================");
    println!("====                 Methods On Enums                     ====");
    println!("==============================================================");

    println!("... we can also define methods on enums");

    let message05 = Message::Write(String::from("world"));
    message05.call();

    println!("==============================================================");
    println!("====                 The Option Enum                      ====");
    println!("==============================================================");

    println!("... the Option enum is a very useful enum that is defined in the standard library");
    println!("... it is used to represent a value that can be either something or nothing");
    println!("... Option, Some and None are already in scope so we don't need to use Option::Some or Option::None\n");

    let some_number = Some(5);

    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    println!("Some Number: {:?}", some_number);

    println!("Some String: {:?}", some_string);

    println!("Absent Number: {:?}", absent_number);

    println!("\n... we can use the `unwrap` method to get the value out of a Some");

    println!("Some Number: {}", some_number.unwrap());
    println!("Some String: {}", some_string.unwrap());

    println!("\n... so what is the difference between 'null' and 'None'?");
    println!("... Rust does not have a null value but it does have an enum called 'Option' that can encapsulate a value or nothing");
    println!("... this means that you can't accidentally use a null value where you shouldn't");
    println!("... the compiler will force you to handle the case where the value is None");
    println!("... so everywhere in code where we are not using Option<T> we can safely assume that a value is present");

    println!("==============================================================");
    println!("====                 Match                                ====");
    println!("==============================================================");

    println!("... the match keyword is a powerful control flow operator that allows you to compare a value against a series of patterns and then execute code based on which pattern matches");
    println!("... it is similar to a switch statement in other languages");
    println!("... the _ is a catchall value that will match any value");
    println!("... we can also use the if let syntax to reduce boilerplate");

    let my_penny_coin = Coin::Penny;
    let my_quarter_coin = Coin::Quarter;
    let my_nickel_coin = Coin::Nickel;
    let my_dime_coin = Coin::Dime;

    println!("\nMy coins are ...");
    println!("... penny: {:?}", my_penny_coin);
    println!("... quarter: {:?}", my_quarter_coin);
    println!("... nickel: {:?}", my_nickel_coin);
    println!("... dime: {:?}", my_dime_coin);

    println!("\n... the value of each is ...");
    println!("... penny : {}", value_in_cents(my_penny_coin));
    println!("... quarter : {}", value_in_cents(my_quarter_coin));
    println!("... nickel : {}", value_in_cents(my_nickel_coin));
    println!("... dime : {}", value_in_cents(my_dime_coin));

    println!("==============================================================");
    println!("====                    Patterns                          ====");
    println!("==============================================================");

    println!("... patterns are a way to match against the structure of types");

    let penny_with_state = CoinWithState::Penny;
    let nickel_with_state = CoinWithState::Nickel;
    let dime_with_state = CoinWithState::Dime;
    let quarter_from_alaska = CoinWithState::Quarter(UsState::Alaska);
    let quarter_from_alabama = CoinWithState::Quarter(UsState::Alabama);


    println!("\nMy coins with nickel state are ...");
    println!("... penny: {:?}", penny_with_state);
    println!("... nickel: {:?}", nickel_with_state);
    println!("... dime: {:?}", dime_with_state);
    println!("... quarter from Alaska: {:?}", quarter_from_alaska);
    println!("... quarter from Alabama: {:?}", quarter_from_alabama);

    println!("\n... the value of each is ...");
    println!("... penny : {}", value_in_cents_with_state(&penny_with_state));
    println!("... nickel : {}", value_in_cents_with_state(&nickel_with_state));
    println!("... dime : {}", value_in_cents_with_state(&dime_with_state));
    println!("... quarter from Alaska : {}", value_in_cents_with_state(&quarter_from_alaska));
    println!("... quarter from Alabama : {}", value_in_cents_with_state(&quarter_from_alabama));

    println!("... given a quarter we can now extract the state");

    let quarter_from_alaska2 = CoinWithState::Quarter(UsState::Alaska);
    let quarter_from_alabama2 = CoinWithState::Quarter(UsState::Alabama);

    println!("\nMy coins with nickel state are ...");
    println!("... quarter from Alaska: {:?}", quarter_from_alaska2);
    println!("... quarter from Alabama: {:?}", quarter_from_alabama2);

    println!("\n... the state of each is ...");
    println!("... quarter from Alaska : {:?}", get_state_from_quarter(&quarter_from_alaska2));
    println!("... quarter from Alabama : {:?}", get_state_from_quarter(&quarter_from_alabama2));

    println!("\n... we can also extract the state name ...");
    println!("... quarter is from the state of : {}", get_state_name_from_quarter(&quarter_from_alaska2));
    println!("... quarter is from the state of : {}", get_state_name_from_quarter(&quarter_from_alabama2));

    println!("==============================================================");
    println!("====                 Option<T> Example                    ====");
    println!("==============================================================");

    println!("... we have a regular function we can add one to a number");

    let pure_number01 = 5;
    println!("Pure Number 01: {}", pure_number01);
    let number02 = add_one(pure_number01);
    println!("Number 02: {}", number02);

    println!("\n... we can also add one to an Option<i32>");

    let some_number = Some(5);
    println!("Some Number: {:?}", some_number);

    let some_number_plus_one = add_one_to_option(some_number);
    println!("Some Number Plus One: {:?}", some_number_plus_one);

    let absent_number: Option<i32> = None;
    println!("Absent Number: {:?}", absent_number);

    let absent_number_plus_one = add_one_to_option(absent_number);
    println!("Absent Number Plus One: {:?}", absent_number_plus_one);

    println!("\n... we can also use the unwrap_or method to provide a default value");

    let absent_number_plus_one_v2 = absent_number.unwrap_or(0);
    println!("Absent Number Plus One V2: {:?}", absent_number_plus_one_v2);

    println!("\n... we can also use the unwrap_or_else method to provide a default value");

    let absent_number_plus_one_v3 = absent_number.unwrap_or_else(|| 0);
    println!("Absent Number Plus One V3: {:?}", absent_number_plus_one_v3);

    println!("\n... we can also manually code a function to add one or return a default value");
    let good_number = Some(5);
    let invalid_number = None;

    let good_number_plus_one = add_one_to_option_v2(good_number);
    println!("Good Number Plus One: {:?}", good_number_plus_one);
    let invalid_number_plus_one = add_one_to_option_v2(invalid_number);
    println!("Invalid Number Returns Default: {:?}", invalid_number_plus_one);
    
    println!("\n... we can also use the map method to transform the value");

    let some_number_plus_one_v2 = some_number.map(|i| i + 1);
    println!("Some Number Plus One V2: {:?}", some_number_plus_one_v2);

    let absent_number_plus_one_v4 = absent_number.map(|i| i + 1);
    println!("Absent Number Plus One V4: {:?}", absent_number_plus_one_v4);

    println!("==============================================================");
    println!("====                    If ... Let                        ====");
    println!("==============================================================");

    println!("... we can also use the if let syntax to reduce boilerplate");

    let some_number = Some(5);
    println!("Some Number: {:?}", some_number);

    if let Some(value) = some_number {
        println!("Some Number: {}", value);
    }

    println!("==============================================================");
    println!("====                   Match All                          ====");
    println!("==============================================================");

    println!("... matches are fully exhaustive, meaning that we must account for every possible case");

    let dice_roll = 9;
    match dice_roll {
        3 => do_this_on_three(),
        7 => do_this_on_seven(),
        other => move_spaces(other), // can use _ if we are not passing the value
    }

    match dice_roll {
        3 => do_this_on_three(),
        7 => do_this_on_seven(),
        _ => (), // do nothing
    }

    println!("==============================================================");
    println!("====               If ... Let  (continued)                ====");
    println!("==============================================================");

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    println!("... we can use if let to reduce the boilerplate");

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    println!("==============================================================");
    println!("====         counting coins ... if ... let example        ====");
    println!("==============================================================");

    println!("let's generate a quick example to count the number of non-quarter coins, and for quarters to print out the state they are from");

    let coin_to_count_01 = CoinWithState::Penny;
    let coin_to_count_02 = CoinWithState::Nickel;
    let coin_to_count_03 = CoinWithState::Dime;
    let coin_to_count_04 = CoinWithState::Quarter(UsState::Alaska);
    let coin_to_count_05 = CoinWithState::Quarter(UsState::Alabama);

    let mut coin_count = 0u32;

    coin_count = count_coins(&coin_to_count_01, coin_count);
    coin_count = count_coins(&coin_to_count_02, coin_count);
    coin_count = count_coins(&coin_to_count_03, coin_count);
    coin_count = count_coins(&coin_to_count_04, coin_count);
    coin_count = count_coins(&coin_to_count_05, coin_count);

    println!("... the total number of coins is: {}\n", coin_count);

    let mut coin_count_v2 = 0u32;

    coin_count_v2 = count_coins_v2(&coin_to_count_01, coin_count_v2);
    coin_count_v2 = count_coins_v2(&coin_to_count_02, coin_count_v2);
    coin_count_v2 = count_coins_v2(&coin_to_count_03, coin_count_v2);
    coin_count_v2 = count_coins_v2(&coin_to_count_04, coin_count_v2);
    coin_count_v2 = count_coins_v2(&coin_to_count_05, coin_count_v2);

    println!("... the total number of coins is: {}\n", coin_count_v2);

    println!("... we can also use the if let syntax to reduce boilerplate");

    let mut coin_count_v3 = 0u32;

    coin_count_v3 = count_coins_v3(&coin_to_count_01, coin_count_v3);
    coin_count_v3 = count_coins_v3(&coin_to_count_02, coin_count_v3);
    coin_count_v3 = count_coins_v3(&coin_to_count_03, coin_count_v3);
    coin_count_v3 = count_coins_v3(&coin_to_count_04, coin_count_v3);
    coin_count_v3 = count_coins_v3(&coin_to_count_05, coin_count_v3);

    println!("... the total number of coins is: {}\n", coin_count_v3);


    println!("==============================================================");
    println!("====                 Summary                              ====");
    println!("==============================================================");

    println!("... structs are a way to define a type with named fields");
    println!("... tuple structs are a way to define a type with unnamed fields");
    println!("... unit-like structs are a way to define a type with no fields");
    println!("... methods are defined within the context of a struct");
    println!("... associated functions are defined within the context of a struct but do not take a reference to the struct as a parameter");
    println!("... enums are a way to define a type by enumerating its possible values");
    println!("... we can also define methods on enums");
    println!("... the Option enum is a very useful enum that is defined in the standard library");
    println!("... the match keyword is a powerful control flow operator that allows you to compare a value against a series of patterns and then execute code based on which pattern matches");
    println!("... matches are fully exhaustive, meaning that we must account for every possible case");
    println!("... patterns are a way to match against the structure of types");
    println!("... the _ is a catchall value that will match any value");
    println!("... we can also use the if let syntax to reduce boilerplate");

}

#[derive(Debug)]
struct MyStruct {
    field01: i32,
    field02: i32,
}

impl MyStruct {
    pub fn new(field01: i32, field02: i32) -> MyStruct {
        MyStruct {
            field01,
            field02,
        }
    }
    fn my_method(&self) -> i32 {
        self.field01 + self.field02
    }
}

#[derive(Debug, Clone)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum CoinWithState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin: &CoinWithState) -> u32 {
    match coin {
        CoinWithState::Penny => {
            println!("... lucky penny");
            1
        },
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("... state quarter from {:?}", state);
            25
        },
    }
}

fn get_state_from_quarter(coin: &CoinWithState) -> Option<UsState> {
    match coin {
        CoinWithState::Quarter(state) => {
            println!("... state quarter from {:?}", state);
            Some(state.clone())
        },
        _ => {
            println!("... not a quarter");
            None
        }
    }
}

fn get_state_name_from_quarter(coin: &CoinWithState) -> String {
    match get_state_from_quarter(coin) {
        Some(state) => format!("{:?}", state),
        None => String::from("Not a quarter"),
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("... lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("... calling the message method");
    }
}

#[derive(Debug)]
enum IpAddr2Enum {
    VVV4(u8, u8, u8, u8),
    VVV6(String),
}

#[derive(Debug)]
enum IpAddrEnum {
    VV4(String),
    VV6(String),
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn has_width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn build_user(email: String, username: String) -> User {
    println!("Building user with email: {} and username: {}", email, username);
    println!("... using field init shorthand syntax");
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn get_area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn get_area_ref(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn route(ip_kind: IpAddrKind) {
    println!("... route to {:?}", ip_kind);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_one_to_option(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_one_to_option_v2(x: Option<i32>) -> i32 {
    match x {
        None => 0,
        Some(i) => i + 1,
    }
}

fn do_this_on_three() {
    println!("... move three spaces");
}

fn do_this_on_seven() {
    println!("... move seven spaces");
}

fn move_spaces(spaces: i32) {
    println!("... move {} spaces", spaces);
}

fn count_coins(coin: &CoinWithState, mut coin_count: u32) -> u32 {
    match coin {
        CoinWithState::Penny  =>  { 
            coin_count += 1; 
            coin_count
        },
        CoinWithState::Nickel =>  { 
            coin_count += 1; 
            coin_count
        },
        CoinWithState::Dime   =>  { 
            coin_count += 1; 
            coin_count
        },
        CoinWithState::Quarter(state) => {
            println!("... state quarter from {:?}", state);
            coin_count
        },
    }
}

fn count_coins_v2(coin: &CoinWithState, mut coin_count: u32) -> u32 {
    match coin {
        CoinWithState::Quarter(state) => {
            println!("... state quarter from {:?}", state);
            coin_count
        },
        _ => { 
            coin_count += 1;
            coin_count
        },
    }
}

fn count_coins_v3(coin: &CoinWithState, mut coin_count: u32) -> u32 {
    if let CoinWithState::Quarter(state) = coin {
        println!("... state quarter from {:?}", state);
        coin_count
    } else {
        coin_count += 1;
        coin_count
    }
}