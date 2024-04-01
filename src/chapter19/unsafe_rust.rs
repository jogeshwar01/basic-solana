// Unsafe Rust - Five abilities of unsafe Rust
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of unions
// - Unsafe doesn't turn off the borrow checker or disable any other of Rust's safety checks

// Check references vs smart pointers vs raw pointers
// Raw pointers are similar to references but have some differences:
// 1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
// 2. Aren't guaranteed to point to valid memory
// 3. Are allowed to be null
// 4. Don't implement any automatic cleanup

static HELLO_WORLD: &str = "Hello, world!";  // must have 'static lifetime (here rust can infer that)
// diff b/w constants and immutable static variables: Static variables have a fixed address in memory. Constants are inlined wherever they're used.

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn unsafe_rust() {
  // create mutable and immutable raw pointers from references
  let mut num = 5;
  let r1 = &num as *const i32;  // here * is not a dereference operator but just pointer type
  let r2 = &mut num as *mut i32;

  // let address = 0x012345usize; // create raw pointer from a memory address
  // let r = address as *const i32;

  // dereference raw pointers
  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);

    *r2 = 10;
    println!("num is: {}", num);
  }

  // Call an unsafe function or method
  unsafe fn dangerous() {
    println!("Dangerous function");
  }

  unsafe {
    dangerous();
  }

  // create safe abstraction over unsafe code
  let mut v = vec![1, 2, 3, 4, 5, 6];
  let r = &mut v[..];
  let (a, b) = r.split_at_mut(3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);

  // Using extern Functions to Call External Code
  // Extern facilitates calling external code written in another language
  extern "C" {  // "C" is the Application Binary Interface (ABI) - defines how to call functions written in C from Rust
    fn abs(input: i32) -> i32;
  }

  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }

  #[no_mangle]  // disable Rust's name mangling
  pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
  }

  // Mutating Static Variables is Unsafe
  add_to_count(3);
  unsafe {
      println!("COUNTER: {}", COUNTER);
  }

  // Implementing an Unsafe Trait - a trait is unsafe when at least one of its methods has some unsafe code
  unsafe trait Foo {
    // methods go here
  }

  unsafe impl Foo for i32 {
    // method implementations go here
  }

  // Unions
  // A union is similar to a struct, but only one declared field is used in a particular instance at one time.
}

// here split_at_must is a safe abstraction over unsafe code - it might look something like this:
// use std::slice;

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }