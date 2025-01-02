mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
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

pub fn eat_at_restaurant() {
    // absolute
    crate::front_of_house::hosting::add_to_waitlist();
    // relative
    front_of_house::hosting::add_to_waitlist();
    // using 'use' still makes it clear it's not a local function
    hosting::add_to_waitlist();
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

    let _order3 = back_of_house::Appetizer::Soup;
    let _order4 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {
    println!("deliver_order");
}
