// created with `cargo new adder --lib`
// to run tests - `cargo test`
// Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed
// a test fails when it panics, when an assertion fails, or when the test times out
// assert_eq! - there is no distinction bw left and right in the assert_eq! macro (that one is the expected value, and the other is the actual value)
// **Also both the expected and actual values must have the same type, and the type must implement the PartialEq and Debug traits

// #[cfg(test)]                         - this attribute tells Rust to compile and run the test code only when we run cargo test
// cargo test --help                    - to see the options for running tests
// cargo test -- --help                 - to see the options for the test binary
// cargo test -- --test-threads=1       - to run tests in serial
// cargo test -- --show-output          - to see the output of passed tests
// cargo test it_works                  - to run only the it_works test
// cargo test add                       - by using part of the function name, we can run all tests that match that name
// cargo test test::                    - to run all tests in the test module
// cargo test -- --ignored              - to run only the ignored tests - ignore a test by adding #[ignore] above the test function

// Binary crates (src/main.rs) cannot have integration tests and cannot be tested with cargo test
// Integration tests are only for library crates (src/lib.rs)
// So it common to have a binary crate thats a thin wrapper around a library crate that contains most of the logic
// The binary crate is then minimally tested, and the logic is tested in the library crate

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;  // use the parent module (this is the test module, so we can use the parent module to access the functions)

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn failing_test() {
    //     panic!("This test will fail")
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // no distinction bw left and right in the assert_eq! macro
        assert_ne!(5, add_two(2)); // assert_ne! is the opposite of assert_eq!
    }

    // custom failure messages
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`", result
    //     );
    // }

    // should_panic attribute
    // #[should_panic(expected = "assertion failed")] // can have custom message
    #[test]
    #[should_panic]
    fn greater_than_100() {
        let result = add(100, 10);
        assert_eq!(result, 101);
    }

    // test return result - allow us to use the ? operator in the test
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}