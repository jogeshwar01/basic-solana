mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_fns_closures;
mod macros;

pub fn chapter19() {
  println!("Chapter 19: Advanced Features");
  unsafe_rust::unsafe_rust();
  advanced_traits::advanced_traits();
  advanced_types::advanced_types();
  advanced_fns_closures::advanced_fns_closures();
  macros::macros();
}