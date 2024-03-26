#[derive(Debug)]
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// enum Option<T> {
//   None,
//   Some(T),
// }

#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      _ => 25,
  }
}

pub fn enum_concepts() {
  let home = IpAddr::V4(127, 0, 0, 1);
  let loopback = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  let q = Message::Quit;
  let mv = Message::Move { x: 1, y: 2 };
  let c = Message::ChangeColor(0, 0, 0);

  println!("home: {:?}", home);
  println!("loopback: {:?}", loopback);
  println!("m: {:?}", m);
  println!("q: {:?}", q);
  println!("mv: {:?}", mv);
  println!("c: {:?}", c);

  // let some_number = Some(5);
  // let some_char = Some('e');
  // let absent_number: Option<i32> = None;

  let x: i8 = 5;
  let y: Option<i8> = Some(5);

  let sum = x + y.unwrap_or(0);
  println!("sum: {}", sum);

  println!("value_in_cents(Coin::Penny): {}", value_in_cents(Coin::Penny));

  // let config_max = Some(3u8);
  // match config_max {
  //     Some(max) => println!("The maximum is configured to be {}", max),
  //     _ => (),
  // }

  let config_max = Some(3u8);
  if let Some(max) = config_max {
      println!("The maximum is configured to be {}", max);
  }
}
