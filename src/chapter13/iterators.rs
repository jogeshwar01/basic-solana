// RUST follows zero-cost abstractions - abstractions that allow you to write code in a general way without any runtime performance penalty
// So loops are not slower than while loops, iterators are not slower than loops, etc.

pub fn iterators(){
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();  // in rust, iterators are lazy ie. they do nothing until you call methods to consume them
  // two other options - v1.into_iter() - to take ownership and v1.iter_mut() -to return mutable references
  for val in v1_iter {
    println!("Got: {}", val);
  }  
}

// all iterators implement the Iterator trait
// pub trait Iterator {
//   type Item;  // associated type, like a generic type but for traits - more on this later (chp 19)
//   fn next(&mut self) -> Option<Self::Item>;
//   // all other methods are provided by default implementations
// }

// These methods are defined as default methods in the Iterator trait
// Adapter methods - methods that allow you to change iterators into different kinds of iterators eg. map, filter, skip, take, etc.
// Consuming adaptors - methods that use up the iterator and return something else. eg. sum, collect, etc.

#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();
  
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();
  
  assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
  let v1 = vec![1, 2, 3];
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  
  assert_eq!(v2, vec![2, 3, 4]);
}

// Iterator with Closures that capture their environment
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
  let shoes = vec![
    Shoe { size: 10, style: String::from("sneaker") },
    Shoe { size: 13, style: String::from("sandal") },
    Shoe { size: 10, style: String::from("boot") },
  ];
  
  let in_my_size = shoes_in_my_size(shoes, 10);
  
  assert_eq!(
    in_my_size,
    vec![
      Shoe { size: 10, style: String::from("sneaker") },
      Shoe { size: 10, style: String::from("boot") },
    ]
  );
}

// CREATING OUR OWN ITERATOR
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    
    if self.count < 6 {
      Some(self.count)
    } else {
      None
    }
  }
}

#[test]
fn calling_next_directly() {
  let mut counter = Counter::new();
  
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), Some(4));
  assert_eq!(counter.next(), Some(5));
  assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
  // ZIP - creates an iterator that will iterate over two other iterators, returning a tuple (pair of values)
  // SKIP - skips the first n elements of an iterator
  // FILTER - creates an iterator that uses a closure to determine whether an element should be yielded or not
  // SUM - consumes the iterator and adds up all the elements
  // MAP - creates an iterator that calls a closure on each element
  let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
  
  assert_eq!(18, sum);
}