// SMART POINTERS
// Strings and Vectors are smart pointers
// Smart pointers are data structures that act like pointers but have additional metadata and capabilities
// Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. 
// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

// Box is used in these cases
// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// TRAIT OBJECT - more in chp 17 - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


#[derive(Debug)]
enum List {  // if you like, check cons list on wikipedia - similar to a linked list
  Cons(i32, Box<List>),  // this will give error if Box is not used -- recursive type List has infinite size
                         // this is because the size of List is not known at compile time
  Nil,
}

use List::{Cons, Nil};

pub fn box_pointer() {
  let b = Box::new(5); // 5 stored on the heap, b is a pointer in stack to the heap memory
  println!("b = {b}");

  // let list = Cons(1, Cons(2, Cons(3, Nil)));
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);
}

// Box vs references

// The reason we use Box instead of a direct reference in cases like the List enum example is fundamentally about how Rust handles memory and type sizes at compile time. Let's break down the key points to understand this better:

// Memory and Type Sizes
// Rust needs to know the size of types at compile time for stack allocation. This is crucial for efficiently managing memory and ensuring safety. However, recursive types like the List enum you mentioned can't have their size determined easily by the compiler because they nest instances of themselves, creating a theoretically infinite size.

// Why Not Direct References?
// References in Rust have a known size (they're basically pointers), but using a plain reference (like &T or &mut T) doesn't work for ownership and type-size reasons in this context:

// Ownership and Lifetimes: Rust's ownership model, coupled with lifetimes, ensures memory safety without a garbage collector. When using references, Rust requires you to specify lifetimes, making the design more complex, especially for recursive types. The ownership model around references does not fit well with self-referential or recursively defined structures without additional lifetime annotations.

// Non-Sized Types: References point to data but don't own it. If we tried to use references directly in the List enum, we would still face the problem of how to store and manage the data those references point to, especially since some of that data could be dynamically sized or part of recursive structures, complicating memory management and safety guarantees.

// Role of Box<T>
// Box<T> is a smart pointer that allocates data on the heap while maintaining a fixed size pointer on the stack. It provides ownership of the heap-allocated data, meaning when the Box goes out of scope, the heap memory it points to is properly deallocated. This solves several problems:

// Fixed Size: Box has a known, compile-time size (the size of a pointer), making it compatible with Rust's need to know type sizes.
// Heap Allocation: It allows storing data of unknown or variable size on the heap, which is perfect for recursive data structures like the List enum.
// Ownership Semantics: By providing ownership over the data it points to, Box aligns with Rust's ownership model, ensuring memory safety and proper cleanup without needing garbage collection.
// Conclusion
// Using Box in recursive types or cases where type sizes cannot be determined at compile time is a design choice that aligns with Rust's principles of memory safety, ownership, and type checking. It simplifies dealing with recursive structures by encapsulating the complexities of heap allocation and ownership, making it an essential tool in Rust's system programming environment.