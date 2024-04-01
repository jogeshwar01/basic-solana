// Associated types
// diff bw associated types and generics: Associated types have only one concrete type for a type class, whereas generics can have multiple concrete types.

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub fn advanced_traits() {
  println!("Advanced Traits");
  assert_eq!(
      Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
      Point { x: 3, y: 3 }
  );

  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();

  // if fly were associated/static functions, we would call them like this:
  // <Human as Pilot>::fly(&person);
  // <Human as Wizard>::fly(&person);
  // Human::fly(&person);

  let w = Wrapper(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);
}

struct Counter {}

// We cannot have another implementation of Iterator for Counter because it would conflict with the first one.0
// this will work if we use generics instead of associated types - using <T> and omitting item
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// Default Generic Type Parameters and Operator Overloading
// Default generic type parameters allow you to define a default type that will be used if the caller doesn't specify a type argument.
// Operator overloading allows you to define the behavior of an operator in a custom way for a custom type.

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { // operator overloading
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters { // using <Meters> to specify the type of the other parameter
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}    

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
      println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
      println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
      println!("*waving arms furiously*");
  }
}

// Supertraits: Specifying Another Trait as a Supertrait
use std::fmt;

trait OutlinePrint: fmt::Display { // anything that implements OutlinePrint must also implement Display to use the to_string method
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct DisplayPoint {
    x: i32,
    y: i32,
}

// must implement Display to use the to_string method
impl fmt::Display for DisplayPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for DisplayPoint {}

// Newtype pattern
// The newtype pattern is a design pattern in Rust that allows you to create a new type to implement a trait for an existing type.
// eg. to implement display for a vector (not allowed as both display and vec are outside crate - we create a wrapper struct around vector)
// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. 
// read more of this if needed - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
