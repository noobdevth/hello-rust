use std::io;

const VOWELS: [&'static str; 5]= ["a", "e", "i", "o", "u"];

fn main() {
  println!("Enter a word to convert to Pig Latin!");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  let first_letter = input.chars().nth(0).unwrap().to_lowercase().to_string();

  let mut word = input.trim().to_string();

  if VOWELS.contains(&&*first_letter) {
    word.push_str("-hay");
  } else {
    word = format!("{}-{}ay", &word[1..], first_letter);
  }

  println!("Result: {}", word)
}
