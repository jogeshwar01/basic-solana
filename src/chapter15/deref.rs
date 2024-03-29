// Implementing the Deref trait allows you to customize the behavior of the dereference operator * 
// (not to be confused with the multiplication or glob operator). 
// By implementing Deref in such a way that a smart pointer can be treated like a regular reference, 
// you can write code that operates on references and use that code with smart pointers too.

use std::ops::Deref;

struct MyBox<T>(T);  // tuple struct with one element

impl <T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl <T> Deref for MyBox<T> {
  type Target = T; // associated types - TBD in chp 19

  fn deref(&self) -> &T {
    &self.0 // as MyBox is a tuple struct with one element, we need to access the value using self.0
  }
}

pub fn deref() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);  // True
  assert_eq!(5, *y); // True
  // assert_eq!(5, y);  // False - Error: mismatched types 

  let y = Box::new(x);  // points to value 5 on the heap, 
  //y is pointing to copy of 5 on the heap instead of ownership of 5 on the stack as we have primitiive type

  assert_eq!(5, *y); // y is a Box, but we can dereference it like a reference

  let y = MyBox::new(x);  // points to value 5 on the stack
  assert_eq!(5, *y); // if deref trait not implemented - error, cannot dereference MyBox
  // code called is actually *(y.deref())
  // deref returns a reference to the value in MyBox, and then * follows that reference to get the value
  // as otherwise ownership of y will be transferred outside of our smart pointer 

  let m = MyBox::new(String::from("Rust"));
  hello(&m); // hello function expects a reference to a string slice, but we can pass a reference to a MyBox<String> as well
  // &MyBox<String> is converted to &String (using deref), and then &String is converted to &str (using deref again)

  // if rust did not have deref coercion, we would have to write code like below
  // hello(&(*m)[..]); // (*m) gets the value inside the MyBox, and & and [..] take a string slice of that value

  // DerefMut trait is similar to Deref, but for mutable references.
}

// Implicit deref coercion
// Deref coercion converts a reference to a type that implements Deref into a reference to a type that Deref can convert the original type into.
// here we are passing a reference to a string slice, but we can pass a reference to a MyBox<String> as well
fn hello(name: &str) {
  println!("Hello, {}!", name);
}

// Rust does deref coercion when it finds types and trait implementations in three cases:
// 1. From &T to &U when T: Deref<Target=U>
// 2. From &mut T to &mut U when T: DerefMut<Target=U>
// 3. From &mut T to &U when T: Deref<Target=U>