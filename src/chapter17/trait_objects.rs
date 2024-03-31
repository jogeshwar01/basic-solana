// Use trait objects when you want to abstract over types that implement a trait
// When you want to store values of different types that implement a trait in a collection

// Mono-morphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled
// This is called static dispatch
// The opposite of static dispatch is dynamic dispatch
// Dynamic dispatch is when the compiler doesn't know all the types that might be used at runtime
// When using trait objects, Rust uses dynamic dispatch

// Object safe trait is a trait that can be used to create trait objects
// Only traits that meet the following rules can be made into trait objects
// Rules for object safe trait: - All methods in the trait must have the following properties:
// 1. The return type isn't Self
// 2. There are no generic type parameters

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>, // dyn - dynamic dispatch
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing a button with width: {}, height: {}, label: {}", self.width, self.height, self.label);
  }
}

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("Drawing a select box with width: {}, height: {}, options: {:?}", self.width, self.height, self.options);
  }
}

pub fn trait_objects() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
      }),
    ],
  };

  screen.run();
}

// same thing with generic + trait bound
// there is one difference though. Here are vector of draw must be homogenous ie. cannot store multiple types of components

// pub struct ScreenGeneric<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T> ScreenGeneric<T>
// where
//   T: Draw,
// {
//   pub fn run(&self) {
//     for component in self.components.iter() {
//       component.draw();
//     }
//   }
// }