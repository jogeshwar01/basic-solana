use std::collections::HashMap;

// ---------------VECTORS------------------

pub fn vectors() {
  let mut vec: Vec<i64> = vec![1,2,3,4,5]; // can also use range like 1..6
  vec.len();
  vec[0];
  vec.push(6);
  vec.remove(0);
  println!("{:?}",vec)
}

// ---------------HASH MAPS------------------

pub fn hash_maps() {
  let mut map = HashMap::new();

  map.insert(0, "Hi");
  map.insert(1, "Hi2");
  println!("{:?}",map);

  match map.get(&0) {  // We have Options here - similar to Promises - can be Some or None
    Some(str) => println!("{}",str),
    None => println!("Doesn't exist in map"),
  }

  match map.get(&2) {  // find a key of 2
    Some(str) => println!("{}",str),
    _ => println!("Doesn't exist in map"),
  }

  map.remove(&0);
  println!("{:?}",map);
}

// ---------------OPTIONS------------------

// None, to indicate failure or lack of value, and
// Some(value), a tuple struct that wraps a value with type T
pub fn divide(dividend: i32, divisor: i32) -> Option<i32> {
  if dividend % divisor != 0 {
    None
  } else {
    Some(dividend / divisor)
  }
}

pub fn options() {
  let divide1: Option<i32> = divide(4,2);
  // Unwrapping a 'Some' variant will extract the value wrapped
  println!("{:?} unwraps to {}",divide1,divide1.unwrap());

  // Unwrapping a None variant will panic!
  // let divide2: Option<i32> = divide(2,3);
  // println!("{:?} unwraps to {}",divide2,divide2.unwrap());
}

// ---------------RESULTS------------------

#[derive(Debug)]
enum MyError {
  Error1
}

// Err, an enum that contains an error code/msg
// Ok(value), A wrapper that contains a value
fn divide_result(dividend: i32, divisor: i32) -> Result<i32, MyError> {
  if dividend % divisor != 0 {
    Err(MyError::Error1)
  } else {
    Ok(dividend / divisor)
  }
}

pub fn results() {
  let divide = divide_result(4,2);
  // let res = divide.expect("we crashed"); // if error print this, else print value safely

  match divide {
    Ok(v) => println!("{}",v),
    Err(v) => println!("{:?}",v)
  }

  // if divide.is_ok(){
  //   println!("{}", divide.unwrap());
  // }

  // println!("{}",divide.unwrap());
  // println!("{}",divide.unwrap_or(100)); // if error then print 100
}