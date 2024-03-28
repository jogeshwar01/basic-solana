// need to use PartialOrd trait to allow > - only works for types that implement the PartialOrd trait
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

struct Point<T> {
  x: T,
  y: T,
}

// does not need to use T (just a name) - can use U
impl<U> Point<U> {
  fn x(&self) -> &U {
      &self.x
  }
}

// impl only available for Point(f64) 
impl Point<f64> {
  fn distance_from_origin(&self) -> f64 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

struct Point2<T, U> {
  x: T,
  y: U,
}

impl<X1, Y1> Point2<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
      Point2 {
          x: self.x,
          y: other.y,
      }
  }
}

enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}

pub fn generics() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest(&char_list);
  println!("The largest char is {}", result);

  // let integer = Point { x: 5, y: 10 };
  let float = Point { x: 1.5, y: 4.0 };
  println!("float.x = {}, float.y = {}", float.x, float.y);

  let integer_and_float = Point2 { x: 5, y: 4.0 };

  let p = Point { x: 5.0, y: 10.0 };
  println!("p.x = {}", p.x());
  println!("Distance from origin: {}", p.distance_from_origin());

  let p1 = Point2 { x: 5, y: 10.4 };
  let p2 = Point2 { x: "Hello", y: 'c' };
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

  let integer = Option::Some(5);
  let float = Option::Some(5.0);

  // at compile time, rust will turn this into 2 different types - Option_i32 and Option_f64
  // enum Option_i32 {
  //     Some(i32),
  //     None,
  // }

  // enum Option_f64 {
  //     Some(f64),
  //     None,
  // }
}