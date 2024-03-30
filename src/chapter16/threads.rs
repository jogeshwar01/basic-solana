// Concurrent programming, where different parts of a program execute independently, and parallel programming, where different parts of a program execute at the same time
// Here we would be looking at both concurrent and/or parallel programming

// native threads - one-to-one model: one operating system thread corresponds to one language thread
// green threads - many-to-many model: many language threads are mapped to many operating system - eg.) 20 green threads to 4 operating system threads 

// Due to runtime overhead, Rust has chosen to use native threads and only include native thread support in the standard library

use::std::thread;

pub fn threads() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread!", i);
      thread::sleep(std::time::Duration::from_millis(1));
    }
  });

  // handle.join().unwrap();
  // if we uncomment the line above, the main thread would wait for the spawned thread to finish before continuing

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(std::time::Duration::from_millis(1));
  }

  // Wait for the spawned thread to finish
  handle.join().unwrap();

  move_closures()
}

// Using move Closures with Threads
pub fn move_closures() {
  let v = vec![1, 2, 3];

  // The move keyword before the closure forces the closure to take ownership of the values it uses
  // this is needed as the spawned thread may outlive the main thread (rust would not know how long the spawned thread would run)
  let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
  });

  // drop(v); // This would cause an error if the closure did not take ownership of v

  handle.join().unwrap();
}