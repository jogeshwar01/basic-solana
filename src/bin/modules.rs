// to define binary crate in rust, add a folder bin and create a file in it

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;
// pub use super::front_of_house::hosting;       // parent module - also public for external use
// use self::front_of_house::hosting;            // current module
// use crate::front_of_house::hosting;           // absolute path - preferred - as we can also tell which module we are using
// use std::collections::HashMap;                // external crate
// use std::io::Result as IoResult;              // standard library - with alias
// use rand::{Rng, ErrorKind::Transient};        // external crate with multiple/specific imports
// use std::io::{self, Write};                   // multiple imports from same module and a module itself
// use std::io::*;                               // import all public part from std::io module

// also see how modules work along the directory structure

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use crate creates a symbolic link in the directory
    add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    pub fn cook_order() {}
}

fn main() {
    println!("Hello World!");
    eat_at_restaurant();

    back_of_house::fix_incorrect_order();
}