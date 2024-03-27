use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;

pub fn error_handling() {
  // panic!("crash and burn") // prefix sith RUST_BACKTRACE=1 to see the stack trace

  let v = vec![1, 2, 3];
  // v[99]; // panic

  enum Result<T, E> {
    Ok(T),
    Err(E),
  }

  let greeting_file_result = File::open("hello.txt");
  // let greeting_file = match greeting_file_result {
  //     Ok(file) => file,
  //     Err(error) => panic!("Problem opening the file: {:?}", error),
  // };

  let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error);
        }
    },
};

// can use closures - more about this later

// let greeting_file_result = File::open("hello.txt").unwrap(); // ok value or panic
let greeting_file_result = File::open("hello.txt").expect("Failed to open hello.txt"); // ok value or panic with custom message

let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
  if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
          panic!("Problem creating the file: {:?}", error);
      })
  } else {
      panic!("Problem opening the file: {:?}", error);
  }
});

}

// propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // using ? - similar to above code
    // let mut username_file_result = File::open("hello.txt")?;
    // let mut username = String::new();

    // username_file_result.read_to_string(&mut username)?;
    // Ok(username)

    // even simpler
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // we cannot use ? in main function as it returns None
    // if we want to use ? in main function, we need to return Result type - using trait object - more about this later

    // let home: IpAddr = "127.0.0.1"
    //     .parse()
    //     .expect("Hardcoded IP address should be valid");

}