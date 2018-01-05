use std::io;

const CELSIUS: &'static str = "C";
const FAHRENHEIT: &'static str = "F";

// C = 5/9 (F-32)
fn to_celsius(temp: f32) -> f32 {
  ((temp - 32.0) * 5.0) / 9.0
}

// F = 1.8 C + 32
fn to_fahrenheit(temp: f32) -> f32 {
  (temp * 1.8) + 32.0
}

fn input(message: &'static str) -> String {
  let mut input = String::new();
  println!("{}", message);

  io::stdin().read_line(&mut input).expect("Failed to read line.");

  return input
}

fn main() {
  println!("Celsius <-> Fahrenheit Converter.");

  loop {
    let temp = input("What is the temperature? (Integer)");

    let temp: u32 = match temp.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("That is not an integer. Try again.");
        continue;
      }
    };

    let unit_from = input("What unit is this temperature in? (C or F)");
    let unit_from = unit_from.to_uppercase();
    let unit_from = unit_from.trim();

    let unit_to = if unit_from == CELSIUS {FAHRENHEIT} else {CELSIUS};

    let result = match unit_from {
      CELSIUS => to_fahrenheit(temp as f32),
      FAHRENHEIT => to_celsius(temp as f32),
      _ => {
        println!("That is not a valid unit. Try again.");
        continue;
      }
    };

    println!("{} {} is {} {}", temp, unit_from, result, unit_to);
  }
}
