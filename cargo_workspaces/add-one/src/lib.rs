use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

// While publishing to crates.io, need to publish each package separately

// to run this file, use the following command in root of workspace:
// cargo run -p add-one OR
// cargo run --package add-one OR
// cargo run --bin add-one

// to run all tests in all crates in the workspace, use the following command in root of workspace:
// cargo test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}