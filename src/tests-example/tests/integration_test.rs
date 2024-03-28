use tests_example;

// every file in the tests directory is compiled as its own separate crate
// no need of #[cfg(test)] here as this is a test file

// to run only this test file, use the command:
// cargo test --test integration_test

// files in subdirectories of the tests directory are not compiled as separate crates
// they are compiled as part of the test crate

mod common;  // look for a file named common.rs or common/mod.rs in the tests directory

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests_example::add_two(2));
}