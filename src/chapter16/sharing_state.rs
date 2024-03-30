// Sharing data between threads
// Shared state is a common source of bugs in concurrent programming. Rust provides a few ways to share state between threads, but they are all designed to be explicit and safe.

// Mutex - Mutual Exclusion
// Mutex is a way to ensure that only one thread can access data at a time. Mutex stands for mutual exclusion. Mutex is a smart pointer that locks data and unlocks it when it goes out of scope. Mutex is a blocking operation, meaning that the thread will wait until it can acquire the lock.
// Using Mutexes to Allow Access to Data from One Thread at a Time
// Rules - Acquire the lock before using the data and release the lock when done using the data

// Arc - Atomic Reference Counting Smart Pointer
// Arc<T> is a type like Rc<T> that is safe to use in concurrent situations

// *** Similar to how RefCell<T> provides interior mutability with Rc, Mutex<T> provides interior mutability and can be used with Arc 
// We can use Mutex<T> to mutate the value inside the Mutex<T> even though we’re not allowed to mutate the Mutex<T> itself. ***
// Refcell is for single-threaded, Mutex is for multi-threaded
// Refcell comes with a risk of creating circular references, Mutex comes with a risk of creating deadlocks

// Send and Sync Traits - check if needed
// Send - indicates that ownership of the type implementing Send can be transferred between threads
// Sync - indicates that it is safe to reference the type implementing Sync from multiple threads

use std::sync::{Arc, Mutex};
use std::thread;

pub fn sharing_state() {
  let m = Mutex::new(5);

  {
      let mut num = m.lock().unwrap();
      *num = 6;
      // MutexGuard goes out of scope and the lock is released. It implements Drop trait to achieve this
  }

  println!("m = {:?}", m);

  mutex_example();
}

fn mutex_example() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
      let counter = Arc::clone(&counter);
      let handle = thread::spawn(move || {
          let mut num = counter.lock().unwrap();

          *num += 1;
      });
      handles.push(handle);
  }

  for handle in handles {
      handle.join().unwrap();
  }

  println!("Result: {}", *counter.lock().unwrap());
}