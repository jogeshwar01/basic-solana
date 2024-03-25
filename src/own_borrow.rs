fn pass_number_by_reference(number: &i8) -> bool {
  number.is_negative()
}

fn pass_number_by_value(number: i8) -> bool {
  number.is_negative()
}

fn pass_vec_by_reference(vec: &Vec<i8>) -> bool {
  vec.is_empty()
}

pub fn borrow() {
  // numbers implement Copy, and so can be passed by value or reference
  let number = 42;

  // does not move number because of borrow
  let is_negative_by_ref = pass_number_by_reference(&number);

  // moves number, which can never be used again
  let is_negative_by_value = pass_number_by_value(number);

  // copy not implemented - must be passed by reference
  let vec = vec![];

  // does not move vec
  let is_empty = pass_vec_by_reference(&vec);

  println!("is_negative_by_value: {}", is_negative_by_value);
  println!("is_negative_by_ref: {}", is_negative_by_ref);
  println!("vec {:?} is_empty: {}", vec, is_empty);
}

pub fn concepts() {
  // ----scope
  {
      let x = 42;

      println!("x: {}", x);
  }

  // println!("x: {}", x); // ERROR: x not in scope

  // ----move
  let a = vec![1, 2, 3]; // a growable array literal
  let b = a;             // move: `a` can no longer be used

  println!("b: {:?}", b);

  // ----borrow
  let alice = Person { age: 8 };
  let bob = &alice; // bob borrows alice

  println!("alice: {:?}\nbob: {:?}", alice, bob);
}