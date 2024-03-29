use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn reference_cycles() {
    // Create a reference cycle - causing a memory leak
    // a -> b -> a -> b -> a -> b -> ...

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);  // borrow_mut returns a mutable reference and we use * to dereference it
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}


// WEAK POINTERS
// -------------
// Children should be dropped when their parent is dropped but not the other way around.
// strong_count and weak_count are used to keep track of the number of strong and weak references to a value.

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // Preventing reference cycles - Adding a Reference from a Child to Its Parent
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn weak_pointers() {
  let leaf = Rc::new(Node {
      value: 3,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![]),
  });

  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
  );

  println!(
      "branch strong = {}, weak = {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch),
  );

  // update parent of leaf to point to branch -> downgrading a strong reference to a weak reference
  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  println!(
      "branch strong = {}, weak = {}",
      Rc::strong_count(&branch), 
      Rc::weak_count(&branch),
  );

  println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

