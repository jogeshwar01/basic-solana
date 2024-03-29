use add_one;  // we use _ instead of - in the name of the crate
use rand;

// to run this file, use the following command in root of workspace:
// cargo run -p adder OR
// cargo run --package adder OR
// cargo run --bin adder

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_one::add_one(num));
}

