pub fn primitive() {
    let unsigned: u8 = 10;
    let signed: i8 = 15;
    let float: f32 = 3.14;

    let letter: char = 'A';
    let emoji: &str = "\u{1F600}"; // :D

    let is_true: bool = true;

    println!("unsigned: {}", unsigned);
    println!("signed: {}", signed);
    println!("float: {}", float);
    println!("letter: {}", letter);
    println!("emoji: {}", emoji);
    println!("is_true: {}", is_true);
}

pub fn compound() { 
    let arr: [i32; 3] = [1, 2, 3];
    let other_arr: [i32; 3] = [100; 3];
    println!("index: {}, length: {}", arr[0], other_arr.len());
    // print structure of array
    println!("{:?}", arr);

    let tuple: (i32, f64, char) = (1, 3.14, 'A');
    println!("tuple.0: {}, tuple.1: {}, tuple.2: {}", tuple.0, tuple.1, tuple.2);
    println!("tuple: {:?}", tuple);

    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

pub fn strings() { 
    let _str: &str = "hello world";
    let mut string: String = String::from("Hello World");
  
    let slice = &string[.. 6]; // index 0 to 5
    println!("{}",slice.len());
  
    string.push('1');
    string.push_str("string");
    string = string.replace("Hello","Bye");
    println!("{}",string)
  }

pub fn mutable() {
  let mut num = 5;
  println!("Original Value : {}",num);
  
  num = 3;
  println!("Mutated Value : {}",num);
}

pub fn slice() {
  let arr: [u8; 4] = [0, 1, 2, 3];
  let slice: &[u8] = &arr[1 .. 3]; // [1,2] don't know the length
  
  borrowing_slice(arr,slice)
}

pub fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
  println!("{:?}",arr);
  println!("{:?}",slice);
  println!("length {}", slice.len());
  println!("{} {}", slice[0], slice[1]);
}