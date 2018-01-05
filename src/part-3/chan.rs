use std::thread;
use std::sync::mpsc;

fn main() {
  let (tx, rx) = mpsc::channel();

  let handle = thread::spawn(move || {
    let val = String::from("Hi!");
    tx.send(val).unwrap();
  });

  let received = rx.recv().unwrap();
  println!("Received: {}", received);

  handle.join().unwrap();
}
