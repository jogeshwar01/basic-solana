pub fn oops(){
  let name = String::from("Parrot");
  let bird = Bird {name, age: 22}; // similar to js, can just use name instead of name: name
  bird.print_name();

  println!("{} {}",bird.can_fly(),bird.is_animal());

  let a: MyEnum = MyEnum::A;
  let b: MyEnum = MyEnum::B(5);
  let c: MyEnum = MyEnum::C { x: 10, y: 20 };
  println!("{:?}",a);
  println!("{:?}",b);
  println!("{:?}",c);

  if let MyEnum::B(val) = b {  // if b is of MyEnum::B type
    println!("{}",val);
  }

  if let MyEnum::C{x,y} = c {
    println!("{} {}",x,y);
  }
}

trait Animal {
  fn can_fly(&self) -> bool;
  fn is_animal(&self) -> bool {
    true
  }
}

pub struct Bird {
  pub name: String,
  pub age: u64
}

impl Bird {
  pub fn print_name(&self) {
    println!("{}",self.name)
  }
}

impl Animal for Bird {
  fn can_fly(&self) -> bool {
      true
  }

  fn is_animal(&self) -> bool {
      false
  }
}

#[derive(Debug)]
enum MyEnum {
  A,
  B (i32),
  C {x: i32, y: i32}
}
