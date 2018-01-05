use std::io;
use std::collections::HashMap;

fn main() {
  println!("Input your integers, separated by space.");

  let mut input = String::new();

  io::stdin().read_line(&mut input).expect("Cannot read line");

  let nums: Vec<u32> = input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

  let sum: u32 = nums.iter().sum();
  let length = nums.len() as f32;
  let mean: f32 = sum as f32 / length;

  println!("The mean (average) is {} ({} / {})", mean, sum, length);

  let mut sorted_nums = nums.clone();
  sorted_nums.sort();

  let middle_index = (length / 2.0).round() as usize - 1;

  let median = match sorted_nums.get(middle_index) {
    Some(num) => num,
    None => panic!("Cannot locate the middle value.."),
  };

  println!(
    "The median (middle value) is {} ({:?})",
    median, sorted_nums
  );

  let mut occurs = HashMap::new();

  for num in nums {
    let count = occurs.entry(num).or_insert(0);
    *count += 1;
  }

  let mut max_num = 0;
  let mut max_occur = 0;

  for (num, occur) in occurs {
    if occur > max_occur {
      max_num = num;
      max_occur = occur;
    }
  }

  println!(
    "The mode (most frequent) is {}, which occurs {} times",
    max_num, max_occur
  );
}
