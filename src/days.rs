use std::io;

// const MONTHS: &'static [&'static str] = &[
//   "January", "Febuary", "March", "April", "May", "June", "July",
//   "August", "September", "October", "November", "December"
// ];

const DAYS: [&'static str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

fn main() {
  println!("Hello, World.");

  println!("{}", DAYS[0]);

  println!("Input a day (1 - 7)");

  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .expect("Cannot read from stdin");

  let day: usize = input.trim().parse().expect("Not a Number");

  if day < 1 && day > 7 {
    println!("Day must be between 1 and 7.");
    panic!();
  }

  println!("Day {} is {}", day, DAYS[day - 1]);
}
