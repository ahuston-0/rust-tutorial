mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn shutup_compiler() {
    println!("rand: {}", use_rand());
    match fn2() {
        Ok(_) => println!("success"),
        Err(e) => println!("error {}", e),
    };
    match fn3() {
        Ok(_) => println!("success"),
        Err(e) => println!("error {}", e),
    };
    front_of_house::restaurant_example();
    front_of_house::shutup_compiler();
}

// an example of modules with overloading types
use std::fmt;
use std::io;

struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

fn fn2() -> io::Result<()> {
    println!("Type something");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("You typed: {}", input.trim());
    Ok(())
}

// alias module imports
use std::io::Result as IoResult;
fn fn3() -> IoResult<()> {
    Ok(())
}

// re-export module for use by external code
// pub use crate::front_of_house::hosting;

use rand::Rng;

fn use_rand() -> u8 {
    let secret_number = rand::thread_rng().gen_range(1..101);
    secret_number
}

// use nested paths to simplify use statements
// use std::{cmp::Ordering, io};

// interesting use of nesting for importing a module and submodule
// import io and io::Write with "use std::io; use std::io::Write;"
// use std::io::{self, Write};

// use globs to bring in everything public
// use std::collections::*;
