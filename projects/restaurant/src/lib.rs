// Showcasing modules, paths and use
use std::cmp::Ordering; // use std::io and std::cmp::Ordering
use std::io::{self, Write}; // use std::io and std::io::Write
use std::collections::*; // use all public items defined in a path

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
        
    pub struct Breakfast {
        pub toast: String,
        seasonal_fuit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fuit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super is like .. in file system
    }
    fn cook_order() {}
}

fn deliver_order() {}

pub use crate::front_of_house::hosting; // Absolute path

pub fn eat_at_resturant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    let _something = hosting::add_to_waitlist(); // Relative path

    crate::front_of_house::hosting::add_to_waitlist(); // Absolute path
    front_of_house::hosting::add_to_waitlist(); // Relative path
}

// Crate is the root module of the crate tree
