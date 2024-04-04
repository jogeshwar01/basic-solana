use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool{
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // Job is a type alias for a Box that holds a closure that the thread pool execute expects

enum Message {
  NewJob(Job),
  Terminate,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver)); 
    // Arc - Share ownership between multiple threads, Mutex - Thread-safe mutable container

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // workers.push(Worker::new(id, receiver)); // error after first iteration, because receiver moved to Worker, so it can't be used again
      // So we'd need to use smart pointers like Arc and Mutex to share the receiver among workers
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  // execute function will take a closure that will execute the code we want to run on a thread in the pool
  pub fn execute<F>(&self, f: F) // self as this is a method
    where
        F: FnOnce() + Send + 'static,
        // FnOnce is a trait that represents a closure that takes no parameters and returns the unit type ()
        // Send trait is implemented by types where it is safe to transfer ownership of values between threads
        // 'static lifetime is the entire duration of the program
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

// graceful shutdown and cleanup
// this will be called when the ThreadPool goes out of scope, and will send a Terminate message to all workers
// eg. Pressing Ctrl+C will cause the program to exit, and the ThreadPool will go out of scope
impl Drop for ThreadPool {
  fn drop(&mut self) {

      for _ in &self.workers {
          self.sender.send(Message::Terminate).unwrap();
      }

      println!("Shutting down all workers.");

      for worker in &mut self.workers {
          println!("Shutting down worker {}", worker.id);

          if let Some(thread) = worker.thread.take() {
              thread.join().unwrap(); // ensure that all threads are done executing before the program exits
          }
      }
  }
}

struct Worker {
  id: usize,
  // thread: thread::JoinHandle<()>, // checked thread::spawn implementation, got JoinHandle from there
  thread: Option<thread::JoinHandle<()>>, // needed for graceful shutdown and cleanup
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
      let thread = thread::spawn(move || loop {  // loop will keep the thread alive
        let job = receiver.lock().unwrap().recv().unwrap();

        match job {
          Message::NewJob(job) => {
            println!("Worker {id} got a job; executing.");
            job();
          }
          Message::Terminate => {
            println!("Worker {id} was told to terminate.");
            break;
          }
        }
    });

    Worker {
        id,
        thread: Some(thread),
    }
  }
}

// to make this the primary crate, moved main.rs to bin/main.rs
// Compiler driven development - we will write code that doesn't compile yet, and then write the code that makes it compile
// we want execute function on ThreadPool to have similar signature as thread::spawn