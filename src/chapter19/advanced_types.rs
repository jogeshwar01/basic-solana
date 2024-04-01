pub fn advanced_types() {
  // Type aliases
  type Kilometers = i32;

  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);

  // Never type - return Ok or Err and continue in error
  let guess = "3";
  let mut go_on = false;
  while go_on {
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => {
        go_on = false;
        num
      },
      Err(_) => continue, // how is this allowed even though guess is u32 - because it returns never type
      // here continue will not return anything and it will move control to next iteration, so guess will not be assigned
    };
  }

  // can also use panic!() to return never type
  // also loop {} will return never type (if no break statement is used)

  // Dynamically sized types and the Sized trait
  // let s1: str = "Hello, world!"; // error: str is unsized
  let s1: &str = "Hello, world!"; // &str is a fat pointer, which is a two-word pointer
  // here the first word is the address of the string data, and the second word is the length of the slice
  // this is because the size of a str value is the size of the address of the string data and the length of the slice

  // DSTs are also known as dynamically sized types, and they are a kind of unsized type
  // The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

  // Sized is a special trait that Rust implements for every type. It’s used to determine whether or not a type’s size is known at compile time.
  // Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:
  
  // fn generic<T>(t: T) {
  //     // --snip--
  // }
  // is actually treated as though we had written this:

  // fn generic<T: Sized>(t: T) {
  //     // --snip--
  // }

  // By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
  fn generic<T: ?Sized>(t: &T) {
      // --snip--
  }

  // A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time. The ?Trait syntax with this meaning is only available for Sized, not any other traits.
  // Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer. In this case, we’ve chosen a reference.
}