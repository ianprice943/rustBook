mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}
mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::front_of_house::serving::serve_order();  //go up the module tree and find the right function to call
        }
        fn cook_order() {}

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

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

// these are idiomatic ways
// similar to a symbolic link in the file system
use crate::front_of_house::hosting;
// relative path example
// use self::front_of_house::hosting;

// this is a non idiomatic way to use the add_to_waitlist function. we lose
// some clarity if we do it this way.
// use crate::front_of_house::hosting::add_to_waitlist;

// however, when bringing in structs, enums, and other items with 'use', it's
// idiomatic to specify the full path
use std::collections::HashMap;

// if we wish to bring in two items with the same name into scope
use std::fmt;
use std::io;
// we must go to just before the name similarity to not have conflicts

// the 'as' keyword can also be used to resolve same name problems
use std::io::Result as IoResult;

// we can re-export names with 'pub' and 'use' to bring an item into scope,
// while also allowing others to use it in their scope
pub use crate::front_of_house::hosting;
// this will error out because we've imported hosting already before

// we can nest paths in order to cut down on all the vertical space we use
// for importing crates
use std::{cmp::Ordering, io};
// this errors because we imported io earlier

// we can also use these nested paths for bringing in a library, and a sub library
use std::io::{self, Write};
// this errors because we imported io earlier

// the glob operator allows us to bring all public items defined in a path into
// scope
use std::collections::*;
// careful using this however, as it can be difficult to tell what names are and are
// not in scope

fn function1() -> fmt::Result {
    
}

fn function2() -> io::Result {

}

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    // Setting an enum to pub makes all its variants public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // the paths can be long at times, so we can use the 'use' keyword to shorten things up
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //add_to_waitlist(); // is this a local function? or from a crate?
}
