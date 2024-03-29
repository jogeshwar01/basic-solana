// Drop trait
// It’s used to customize what happens when a value is about to go out of scope.

// use std::mem::drop; // no need to import as it is a prelude

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
  }
}

pub fn drop_test() {
  let c = CustomSmartPointer {
      data: String::from("my stuff"),
  };
  let d = CustomSmartPointer {
      data: String::from("other stuff"),
  };
  println!("CustomSmartPointers created.");

  let e = CustomSmartPointer {
    data: String::from("some data"),
  };
  println!("CustomSmartPointer created.");
  // e.drop(); // error - explicit use of destructor method
  drop(e);  // this is different from the drop method we are implementing, this is provided by the standard library
  println!("CustomSmartPointer dropped before the end of main.");
}

// Some cases might require manual cleanup, and the Drop trait provides a way to implement that.
// The Drop trait requires implementing a method called drop that takes a mutable reference to self.
// Example.) using smart pointers to manage locks, threads, and other resources that require cleanup when they go out of scope.