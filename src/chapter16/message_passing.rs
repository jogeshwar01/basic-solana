// Message Passing - Passing data between threads
// Do not share data between threads, instead pass data between threads
// Channels - A channel is a way to send multiple values from one part of your code to another
// Here we send a value from spawned thread to main thread

use std::sync::mpsc; // Multiple Producer, Single Consumer
use std::thread;

pub fn message_passing() {
  println!("Message Passing");

  // Create a channel
  let (tx, rx) = mpsc::channel();

  // Spawn a thread
  thread::spawn(move || { // need to use the move - as we don't want to share the tx (Sender) value between threads
    let val = String::from("hi");
    tx.send(val).unwrap(); // in production code, we would handle the unwrap error - send takes ownership of the value
    // println!("Sent: {}", val) // this would not work as val has been moved to the spawned thread as we might modify the value in the spawned thread
  });

  // Receive the value from the spawned thread
  let received = rx.recv().unwrap(); 
  // try_recv() does not block main thread, returns Result type immediately (both would return a Result<T, E> - which we can use to handle the error)
  // recv() would block the main thread until a value is sent and would show an error if the thread sending the value panics

  println!("Got: {}", received);

  send_multiple_values();
  multiple_producers();
}

fn send_multiple_values() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(std::time::Duration::from_millis(100));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}

fn multiple_producers() {
  // Multiple Producers
  // Create a channel
  let (tx, rx) = mpsc::channel();

  let tx1 = tx.clone();
  thread::spawn(move || {
    let vals = vec![
      String::from("hi"),
      String::from("from"),
      String::from("the"),
      String::from("thread"),
    ];

    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(std::time::Duration::from_millis(100));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      String::from("more"),
      String::from("messages"),
      String::from("for"),
      String::from("you"),
    ];

    for val in vals {
      tx.send(val).unwrap();
      thread::sleep(std::time::Duration::from_millis(100));
    }
  });

  for received in rx {
    println!("Got: {}", received);
  }
}