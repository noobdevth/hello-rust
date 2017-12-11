extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn input(message: &'static str) -> String {
  println!("{}", message);

  let mut text = String::new();

  io::stdin()
    .read_line(&mut text)
    .expect("Failed to read line");

  return text;
}

fn main() {
  println!("Guess the number!");

  let secret = rand::thread_rng().gen_range(1, 101);

  loop {
    let guess = input("Please Input Your Guess.");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("That's not a number. Try again.");
        continue;
      }
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret) {
      Ordering::Less => println!("Too Small..."),
      Ordering::Greater => println!("Too Big!"),
      Ordering::Equal => {
        println!("You Win!");
        break;
      }
    }
  }
}
