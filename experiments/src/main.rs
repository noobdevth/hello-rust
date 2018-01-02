extern crate time;

use std::thread;
use std::time::Duration;

fn main() {
  let new_year = 1514739600;

  loop {
    let now = time::now().to_timespec();
    let diff = new_year - now.sec;

    if diff > 0 {
      println!("Now is {}", now.sec);
      println!("{} seconds left until new year!", diff);
      thread::sleep(Duration::from_millis(1000));
    } else {
      println!("HAPPY NEW YEAR 2018!");
      break;
    }
  }
}

