mod release_profiles;
// use my_crates::kinds::PrimaryColor;
// use my_crates::utils::mix;

// as we used re-exporting in lib.rs, we can now use the following:
use my_crates::PrimaryColor;
use my_crates::mix;

// Publish the crate to crates.io
// login with github account on crates.io and go to account settings, create a new token, and run `cargo login <token>`
// before publishing, important to check metadata in Cargo.toml
// for eg. name should be unique, version should be updated, description should be there, license should be there, etc.
// run `cargo publish` to publish the crate

// To update the crate, update the version in Cargo.toml and run `cargo publish` again

// To stop publishing the crate, run `cargo yank --vers <version>`
// This will remove the crate from the index (new people cannot download it), but the crate will still be available to those who have already downloaded it
// use --undo to undo the yank

fn main() {
    println!("Hello, world!");

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

