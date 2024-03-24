// use crate::functions::is_even;
mod functions;
mod types;
mod flow;
mod oops;
mod standard;

fn main() {
  println!("Hello World!");
  
  functions::is_even(11);

  types::primitive();
  types::compound();
  types::strings();
  types::mutable();
  types::slice();

  flow::if_statement();
  flow::loops();
  flow::match_case();

  oops::oops();

  standard::vectors();
  standard::hash_maps();
  standard::options();
  standard::results();
}