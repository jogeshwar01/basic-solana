// lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
      x
  } else {
      y
  }
}

// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//   x
// }
// Here return value will live as long as x, as x is returned
// Also the return value must be dependent on atleast one of the input values, as we cannot return a reference created within the function as local variables are destroyed after the function returns
// if we want to return a value from inside fn, use own types like String, Vec, etc so that the value is moved/transferred to the calling function

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Structs with lifetime annotations - need to specify lifetime for each reference in the struct
struct ImportantExcerpt<'a> {
  part: &'a str,
}

// Input and Output lifetime parameters - basically lifetimes of args and return values

// Lifetime ellission rules
// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. Only applies to methods - If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

// Lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
  fn level(&self) -> i32 {
      3
  }
}

impl<'a> ImportantExcerpt<'a> {
  fn announce_and_return_part(&self, announcement: &str) -> &str { // Rule 3 - lifetime of return value is same as self, also self and announcement both have their own lifetimes
      println!("Attention please: {}", announcement);
      self.part
  }
}

// STATIC LIFETIME - lives for the entire duration of the program
// let s: &'static str = "I have a static lifetime.";

pub fn lifetimes() {
  println!("Lifetimes");
  
  // let r;          // ---------+-- 'a
  //                       //          |
  // {                     //          |
  //     let x = 5;   // -+-- 'b  |
  //     r = &x;           //  |       |
  // }                     // -+       |
  //                       //          |
  // println!("r: {}", r); //          |
// }                       // ---------+

  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // ERROR: string2 does not live long enough as lifetime of result is the same as the smaller of the lifetimes of the values referred to by the function arguments
  // let string1 = String::from("long string is long");
  // let result;
  // {
  //     let string2 = String::from("xyz");
  //     result = longest(string1.as_str(), string2.as_str());
  // }
  // println!("The longest string is {}", result);
}


// Everything in one place
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}