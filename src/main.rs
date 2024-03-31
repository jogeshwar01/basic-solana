// use crate::functions::is_even;
mod functions;
mod types;
mod flow;
mod oops;
mod standard;
mod own_borrow;
mod structs;
mod enums;
mod collections;
mod error;
mod chapter10;
mod chapter13;
mod chapter15;
mod chapter16;
mod chapter17;
mod chapter18;

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

  collections::vectors();
  collections::strings();
  collections::hash_maps();

  error::error_handling();

  chapter10::chapter10();
  chapter13::chapter13();
  chapter15::chapter15();
  chapter16::chapter16();
  chapter17::chapter17();
  chapter18::chapter18();
}