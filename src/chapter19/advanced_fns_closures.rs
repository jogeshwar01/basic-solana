// Function pointer - pass a function as an argument to another function
// Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with Fn instead of using one of the Fn traits in trait bound
// Three traits - Fn, FnMut, and FnOnce - that represent the different capabilities a closure must have to capture variables from the environment
// Function pointers implement all three of the Fn traits, so you can always pass a function pointer as an argument for a function that expects a closure
// It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
// one example of where you would want to only accept fn and not closures is when interfacing with external code that doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

pub fn advanced_fns_closures() {
  let answer = do_twice(add_one, 5);

  println!("The answer is: {}", answer);

  // let list_of_numbers = vec![1, 2, 3];
  // let list_of_strings: Vec<String> =
      // list_of_numbers.iter().map(|i| i.to_string()).collect();

  // same code as above but using function instead of closure

  let list_of_numbers = vec![1, 2, 3];
  let list_of_strings: Vec<String> =
      list_of_numbers.iter().map(ToString::to_string).collect();

  enum Status {
      Value(u32),
      Stop,
  }

  let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
  // here Value is treated as a function pointer where the arg will be u32 and return will be Value variant

  // // Returning closures
  // fn returns_closure() -> dyn Fn(i32) -> i32 {
  //     |x| x + 1
  // }

  fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
      Box::new(|x| x + 1)
  }

}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
  where T: Fn(i32) -> i32
{
  f(arg) + f(arg)
}




// fn add_one(x: i32) -> i32 {
//   x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//   f(arg) + f(arg)
// }
