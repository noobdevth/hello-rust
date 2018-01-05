use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::Debug;

#[derive(Debug)]
struct Node<T: Debug> {
  value: T,
  parent: RefCell<Weak<Node<T>>>,
  children: RefCell<Vec<Rc<Node<T>>>>
}

impl<T> Drop for Node<T> where T: Debug {
  fn drop(&mut self) {
    println!("Dropping `Node` with value of {:?}", self.value);
  }
}

fn main() {
  let leaf = Rc::new(Node {
    value: 32,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![])
  });

  println!("Leaf's Parent is {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![leaf.clone()])
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  println!("Leaf's Parent is now {:?}", leaf.parent.borrow().upgrade());
}
