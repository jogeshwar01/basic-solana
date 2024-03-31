pub fn pattern_syntax() {
  // Basic match

  let x = 1;
  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // Match on named variables
  let x = Some(5);
  let y = 10;

  // match block creates a new scope
  match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y), // this is not the same as the y outside the match block
    _ => println!("Default case, x = {:?}", x),
  }
  println!("At the end: x = {:?}, y = {:?}", x, y);

  // Multiple patterns
  let x = 1;
  match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
  }

  // Matching ranges - range operator only works with numeric values or char
  let x = 5;
  match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
  }

  // Destructuring structs
  struct Point {
    x: i32,
    y: i32,
  }

  let p = Point { x: 0, y: 7 };
  let Point { x: a, y: b } = p;
  assert_eq!(0, a);
  assert_eq!(7, b);

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis at ({}, {})", x, y),
  }

  // Desctructuring enums
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  let msg = Message::ChangeColor(0, 160, 255);
  match msg {
    Message::Quit => {
      println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
      println!("Move in the x direction {} and in the y direction {}", x, y);
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b);
    }
  }

  // Destructuring Nested Structs and Enums
  enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
  }

  enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
  }

  let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
  match msg {
    Message2::ChangeColor(Color::Rgb(r, g, b)) => {
      println!("Change the color to red {}, green {}, and blue {}", r, g, b);
    }
    Message2::ChangeColor(Color::Hsv(h, s, v)) => {
      println!("Change the color to hue {}, saturation {}, and value {}", h, s, v);
    }
    _ => (),
  }

  // more complicated destructuring
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

  // Ignoring values in a pattern
  fn foo(_: i32, y: i32) { // useful when we need a fn signature to be something but we don't need to use the parameter
    println!("This code only uses the y parameter: {}", y); 
  }

  let mut setting_value = Some(5);
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
      (Some(_), Some(_)) => {
          println!("Can't overwrite an existing customized value");
      }
      _ => {
          setting_value = new_setting_value;
      }
  }

  println!("setting is {:?}", setting_value);

  let numbers = (2, 4, 8, 16, 32);
  match numbers {
      (first, _, third, _, fifth) => {
          println!("Some numbers: {first}, {third}, {fifth}")
      }
  }

  let _x = 5;
  let y = 10;

  let s = Some(String::from("Hello!"));

  // if let Some(_s) = s {
  //     println!("found a string");
  // }

  // if we dont want to move the value inside Some use only _
  if let Some(_) = s {
      println!("found a string");
  }

  println!("{:?}", s);

  // Range syntax in struct/tuple
  struct Point3D {
    x: i32,
    y: i32,
    z: i32,
  }

  let origin = Point3D { x: 0, y: 0, z: 0 };

  match origin {
    Point3D { x, .. } => println!("x is {}", x),
  }

  let numbers = (2, 4, 8, 16, 32);

  match numbers {
      (first, .., last) => {
          println!("Some numbers: {first}, {last}");
      }
  }

  // Range syntax cannot be ambiguous
  // match numbers {
  //     (.., second, ..) => {
  //         println!("Some numbers: {}", second)
  //     },
  // }

  // Match guards
  let num = Some(4);
    
  match num {
      Some(x) if x < 5 => println!("less than five: {x}"),
      Some(x) => println!("{x}"),
      None => (),
  }

  let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"), // used n to not shadow y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // x == 4 or 5 or 6 and y == true
        _ => println!("no"),
    }

    // @ Bindings
    enum Message3 {
      Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_variable @ 3..=7 } => { // check if id is in the range 3 to 7 and bind the value to id_variable
            println!("Found an id in range: {id_variable}")
        },
        Message3::Hello { id: 10..=12 } => { // can use range operator directly if we don't need to bind the value
            println!("Found an id in another range")
        },
        Message3::Hello { id } => {
            println!("Found some other id: {id}")
        },
    }

}