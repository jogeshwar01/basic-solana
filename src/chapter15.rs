mod box_pointer;
mod deref;
mod drop;
mod reference_counting;
mod interior_mutability;
mod reference_cycles;

pub fn chapter15() {
  println!("Chapter 15: Smart Pointers");
  box_pointer::box_pointer();
  deref::deref();
  drop::drop_test();
  reference_counting::reference_counting();
  interior_mutability::interior_mutability();
  reference_cycles::reference_cycles();
  reference_cycles::weak_pointers();
}