
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn make_summer_breakfast(toast: &str) -> Breakfast {
            prepare_seasonal_fruit();
            fix_incorrect_order();
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn get_seasonal_fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }



    fn prepare_seasonal_fruit() {
        println!("prepare_seasonal_fruit");
    }

    fn cook_order() {
        println!("cook_order");
    }
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        pub fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("take_order");
        }

        pub fn serve_order() {
            println!("serve_order");
        }

        pub fn take_payment() {
            println!("take_payment");
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    crate::front_of_house::hosting::seat_at_table();
    crate::front_of_house::serving::take_order();

    let mut meal = back_of_house::Breakfast::make_summer_breakfast("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries");
    println!("This breakfast comes with {}", meal.get_seasonal_fruit());

    // back_of_house::fix_incorrect_order();

    // Order a breakfast in the summer with Rye toast
    let _order1 = back_of_house::Breakfast::make_summer_breakfast("Rye");

    // Order a breakfast in the summer with Wheat toast
    let _order2 = back_of_house::Breakfast::make_summer_breakfast("Wheat");

    crate::front_of_house::serving::serve_order();

    crate::deliver_order();

    crate::front_of_house::serving::take_payment();
}

fn deliver_order() {
    println!("deliver_order");
}

