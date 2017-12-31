extern crate rand;
extern crate time;

use std::sync::mpsc;
use std::thread;
// use time::PreciseTime;
// use rand::Rng;

fn thread() {
  match result {
    Ok => 1,
    _ => 2
  }

  let vec = Vec::new()

  static THREADS: i32 = 100;

  let (sender, receiver) = mpsc::channel();

  for id in 0..THREADS {
    let thread_send = sender.clone();

    thread::spawn(move || {
      thread_send.send(id).unwrap();
      println!("Thread {} Finished.", id);
    });
  }

  let mut ids = Vec::with_capacity(THREADS as usize);

  for _ in 0..THREADS {
    ids.push(receiver.recv().unwrap());
  }

  println!("Result: {:?}", ids);
}

fn main() {
  println!("==== Aethereal v1.0.0 ====");
  println!("> [INIT] Bootstrapping Aethereal");

  thread();
}

