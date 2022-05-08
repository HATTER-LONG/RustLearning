mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {
            super::hosting::add_to_waitlist();
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // error : meal.seasonal_fruit = String::from("banana");
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod demo {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod use_demo {
    pub mod hosting {
        pub fn add_waitlist() {}
    }
}

pub use crate::use_demo::hosting as demoHosting;

pub fn eat_somgthing() {
    demoHosting::add_waitlist();
}

use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;

use std::io::{self, Write};

use std::collections::*;
