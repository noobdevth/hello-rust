use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  End
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match *self {
      Cons(_, ref item) => Some(item),
      End => None
    }
  }
}

use List::{Cons, End};

fn main() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(End))));
  println!("A {:?} {:?}", &Rc::strong_count(&a), &a.tail());

  let b = Rc::new(Cons(10, RefCell::new(a.clone())));
  println!("A after B {:?}", Rc::strong_count(&a));
  println!("B {:?} {:?}", &Rc::strong_count(&b), &b.tail());

  if let Some(ref link) = a.tail() {
    *link.borrow_mut() = b.clone();
  }

  println!("B after A {:?}", Rc::strong_count(&b));
  println!("A after A {:?}", Rc::strong_count(&a));

  // println!("OMG Overflow {:?}", a.tail());
}

