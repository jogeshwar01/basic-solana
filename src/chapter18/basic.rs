pub fn basic() {
  // =================================
  // Match Arms
  // =================================
  #[derive(Debug)]
  enum Language {
    English,
    Spanish,
    French,
    German,
  }

  let lang = Language::English;
  match lang {
    Language::English => println!("Hello"),
    Language::Spanish => println!("Hola"),
    Language::French => println!("Bonjour"),
    // _ => println!("I don't know that language"),  
    // The underscore (_) is a catch-all pattern that will match any value.
    // Note: underscore does not bind the value to a variable. If we want to bind the value to a variable, we can use a variable name instead of an underscore.
    language => println!("I don't know that language: {:?}", language),
  }

  // =================================
  // Conditional if-let
  // =================================

  let authorization_status: Option<&str> = None;
  let is_admin = false;
  let group_id: Result<u8,_> = "34".parse();

  if let Some(status) = authorization_status {
    println!("Authorization status: {}", status);
  } else if is_admin {
    println!("Admin user");
  } else if let Ok(group) = group_id {
    if group > 30 {
      println!("Group ID is greater than 30");
    } else {
      println!("Group ID is less than or equal to 30");
    }
  } else {
    println!("No authorization status");
  }

  // =================================
  // while let conditional loop
  // =================================

  let mut stack = vec![1, 2, 3, 4, 5];
  while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
  }

  // =================================
  // for loops
  // =================================
  let v = vec!['a', 'b', 'c', 'd', 'e'];
  for (index, value) in v.iter().enumerate() {  // pattern to destructure the tuple
    println!("{} is at index {}", value, index);
  }

  // =================================
  // let statements - let PATTERNS = EXPRESSION;
  // =================================
  let (a, b) = (1, 2);

  // =================================
  // Function params
  // =================================
  fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
  }

  let point = (3, 5);
  print_coordinates(&point);

  // =================================
  // Irrrefutable patterns - always match
  // Refutable patterns - may not match
  // =================================

  let x = 5;

  let x : Option<&str> = None;
  // let Some(y) = x;  // error: refutable pattern in local binding: `None` not covered

  // The following can only be used with irrefutable patterns
  // 1. function params
  // 2. let statements
  // 3. for loops

  // The following can be used with refutable patterns
  // 1. if let
  // 2. while let
  // These will give a warning if the pattern is irrefutable
}