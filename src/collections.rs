pub fn vectors() {
  let mut v2 = Vec::new();
  v2.push(5);
  v2.push(6);

  let v = vec![1, 2, 3, 4, 5];
  // let v: Vec<i32> = Vec::new();

  let third: &i32 = &v[2];
  // v.push(6); // ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable

  println!("The third element is {third}");

  let third: Option<&i32> = v.get(2);
  match third {
      Some(third) => println!("The third element is {third}"),
      None => println!("There is no third element."),
  }

  // dereference operator
  let mut v = vec![100, 32, 57];
  for i in &mut v {
      *i += 50;
  }

  // vector with multiple types
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
  ];

  match &row[1] {
    SpreadsheetCell::Text(value) => println!("Text: {}", value),
    _ => println!("Not a text"),
  }

}

pub fn strings () {
  // strings are stored as a collection of UTF-8 encoded bytes
  let mut s = String::new();
  let data = "initial contents";
  let s = data.to_string();
  let s = "initial contents".to_string();
  let s = String::from("initial contents");

  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s1 is {s1}");

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used while s2 is being appended

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{s1}-{s2}-{s3}"); // format! does not take ownership of any of its parameters

  // let s1 = String::from("hello");
  // let h = s1[0]; // ERROR: cannot index into a string

  // let hello = "Здравствуйте";
  // let answer = &hello[0]; // error: byte index 0 is not a char boundary; it is inside 'З'

  // problem because rust does not know how many bytes each character takes as strings could be of diffrent types
  // namaste can be written as
  // bytes - [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
  // scalar values - ['न', 'म', 'स', '्', 'त', 'े']
  // grapheme clusters - ['न', 'म', 'स्', 'ते']

  let hello = "Здравствуйте";
  let s = &hello[0..4];

  for c in "Зд".chars() { // scalar values
      println!("{c}");
  }
  for b in "Зд".bytes() {  // bytes
      println!("{b}");
  }
  // grapheme clusters are not supported by the standard library
  // need to use crate unicode-segmentation
  // for gc in "नमस्ते".graphemes(true) {
  //     println!("{gc}");
  // }

}

pub fn hash_maps() { 
  use std::collections::HashMap;

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name = String::from("Blue");
  let score = scores.get(&team_name);

  for (key, value) in &scores {
      println!("{key}: {value}");
  }

  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25); // overwrites the previous value

  // only insert if the key does not exist
  scores.entry(String::from("Yellow")).or_insert(50);
  scores.entry(String::from("Blue")).or_insert(50);

  println!("{:?}", scores);

  let text = "hello world wonderful world";

  let mut map = HashMap::new();

  for word in text.split_whitespace() {
      let count = map.entry(word).or_insert(0);
      *count += 1;
  }

  println!("{:?}", map);
}