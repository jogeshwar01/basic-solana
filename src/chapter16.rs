mod threads;
mod message_passing;
mod sharing_state;

pub fn chapter16() {
  println!("Chapter 16: Concurrency");
  threads::threads();
  message_passing::message_passing();
  sharing_state::sharing_state();
}