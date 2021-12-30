mod front_of_house {
    // pub used for giving public access
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Recursive namespaces are allowed
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        mod back_of_house {
            fn fix_incorrect_order() {
                cook_order();
                super::serve_order(); // Reach up through namespaces
            }

            fn cook_order() {}

            pub struct Breakfast {
                // public struct
                pub toast: String,      // public toast var
                seasonal_fruit: String, // private fruit
            }

            pub enum Appetizer {
                Soup,
                Salad,
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

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // Rel path
    front_of_house::hosting::add_to_waitlist();

    // Order with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Cannot change meal.seasonal_fruit

    // Use scoped enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Use "use" keyword with modules as namespaces in C++
    // Typically this is done outside of the function
    use self::front_of_house::hosting;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Example of using std modules
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("Table 1", order1);
}

// an example of modules with overloading types
use std::fmt;
use std::io;

fn fn1() -> fmt::Result {
    // snip
}

fn fn2() -> io::Result<()> {
    // snip
}

// alias module imports
use std::io::Result as IoResult;
fn fn3() -> IoResult<()> {
    // snip
}

// re-export module for use by external code
pub use crate::front_of_house::hosting;

use rand::Rng;

fn use_rand() -> u8 {
    let secret_number = rand::thread_rng().gen_range(1..101);
    secret_number
}

// use nested paths to simplify use statements
use std::{cmp::Ordering, io};

// interesting use of nesting for importing a module and submodule
// import io and io::Write with "use std::io; use std::io::Write;"
use std::io::{self, Write};

// use globs to bring in everything public
use std::collections::*;
