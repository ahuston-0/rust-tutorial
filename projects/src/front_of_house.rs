// pub used for giving public access
pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}

    pub fn shutup_compiler() {
        seat_at_table();
    }
}

// Recursive namespaces are allowed
mod serving {
    fn take_order() {}

    fn serve_order() {}

    pub mod back_of_house {
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

            pub fn get_fruit(&self) -> String {
                self.seasonal_fruit.to_string()
            }
        }

        pub fn shutup_compiler() {
            fix_incorrect_order();
            cook_order();
        }
    }

    fn take_payment() {}

    pub fn shutup_compiler() {
        take_order();
        serve_order();
        take_payment();
        back_of_house::shutup_compiler();
    }
}

pub fn restaurant_example() {
    // Abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // Rel path
    self::hosting::add_to_waitlist();

    // Order with rye toast
    let mut meal = serving::back_of_house::Breakfast::summer("Rye");

    // Change mind about bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    println!("The meal comes with {} by default", meal.get_fruit());
    // Cannot change meal.seasonal_fruit

    // Use scoped enum
    let order1 = serving::back_of_house::Appetizer::Soup;
    let order2 = serving::back_of_house::Appetizer::Salad;

    match order2 {
        serving::back_of_house::Appetizer::Salad => println!("order has salad"),
        serving::back_of_house::Appetizer::Soup => println!("order has soup"),
    }

    // Use "use" keyword with modules as namespaces in C++
    // Typically this is done outside of the function
    // use self::front_of_house::hosting;

    // Example of using std modules
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("Table 1", order1);
}

pub fn shutup_compiler() {
    hosting::shutup_compiler();
    serving::shutup_compiler();
}
