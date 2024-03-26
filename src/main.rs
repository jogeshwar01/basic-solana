// use crate::functions::is_even;
mod functions;
mod types;
mod flow;
mod oops;
mod standard;
mod own_borrow;
mod structs;
mod enums;

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

  own_borrow::borrow();
  own_borrow::concepts();

  structs::get_area();
  enums::enum_concepts();
}