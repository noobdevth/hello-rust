use std::io;

fn fib(num: u32) -> u32 {
  if num <= 1 {
    return num;
  }

  fib(num - 1) + fib(num - 2)
}

fn main() {
  println!("Nth Fibonacci Generator: Enter an integer.");
  let mut num = String::new();

  io::stdin().read_line(&mut num).expect("Failed to read line");

  let num: u32 = num.trim().parse().expect("Not an integer");

  let result = fib(num);

  println!("The {}th fibonacci number is {}.", num, result);
}
