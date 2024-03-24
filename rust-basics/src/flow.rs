pub fn if_statement() {
  let n = 3;

  if n > 0 {
    println!("Greater than 0");
  } else if n < 0 {
    println!("Less than 0");
  } else {
    println!("Is 0");
  }
}

pub fn loops(){
  // for loop
  for i in 0..6 {
    println!("{}",i)
  }

  // while loop
  let mut i = 0;
  while i < 4 {
    println!("{}",i);
    i += 1;
    if i == 3 {
      println!("exit");
      break // no semicolon here
      //continue
    }
  }
}

pub fn match_case() {
  let i = 5;
  match i {
    0 => println!("0"),
    1 | 2 => println!("1,2"),
    3..=4 => println!("3,4"),
    _ => println!("default")
  }
}