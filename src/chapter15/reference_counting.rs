// Reference counting is a way to keep track of the number of references to a value. When the number of references to a value is zero, the value can be deallocated. This is a way to manage memory without using a garbage collector.
// Used when ownership is not clear or when a single value has multiple owners.
// If we already knew the ownership (which part would finish last), we would not need reference counting. We could make the variable that owns the value the owner and let Rust’s ownership system handle the memory deallocation.

// Rc<T> is a reference counting smart pointer provided by the standard library.
// This is only for use in single-threaded scenarios. We'd use Arc<T> for multi-threaded scenarios. TBD in chapter 16.
// Rc::clone only increases the reference count and gives owned value (does not deep copy the data)
// Rc only allows immutable borrows. We can use RefCell for interior mutability. Else borrow checker will not allow us to mutate the value.
enum List {
  Cons(i32, Rc<List>),
  Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

// pub fn reference_counting_util() {
//   let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//   let b = Cons(3, Rc::clone(&a)); // clone increases the reference count (does not deep copy the data)
//   let c = Cons(4, Rc::clone(&a));
// }

pub fn reference_counting() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
      let c = Cons(4, Rc::clone(&a));
      println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}