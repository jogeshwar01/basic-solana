// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
// normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.
// an example - Some properties of code are impossible to detect by analyzing the code: the most famous example is the Halting Problem, which is beyond the scope of this book but is an interesting topic to research.

// Check - Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

use std::cell::RefCell;

pub fn interior_mutability() {
  let x = 5;
  // let y = &mut x; // error: cannot borrow `x` as mutable, as it is not declared as mutable
  // println!("y: {}", y);

  let mut a = 5;
  let d = &a;
  // *d = 6; // error: cannot assign to `*d` because it is borrowed 
  // not allowed at compile - update mutable value of immutable references. Can be done using RefCell<T>

  let x = RefCell::new(42);
  let y = x.borrow_mut();
  println!("y: {:?}", y);
}

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {  // T must implement Messenger trait
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
      LimitTracker {
          messenger,
          value: 0,
          max,
      }
  }

  pub fn set_value(&mut self, value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
          self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
          self.messenger
              .send("Urgent warning: You've used up over 90% of your quota!");
      } else if percentage_of_max >= 0.75 {
          self.messenger
              .send("Warning: You've used up over 75% of your quota!");
      }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));  // borrow_mut() returns mutable reference to the value inside RefCell
        
            // The following code will compile due to RefCell but would panic at runtime because it violates the borrowing rules
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);


        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // borrow() returns immutable reference to the value inside RefCell
    }
}
