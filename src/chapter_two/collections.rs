pub mod counts {
  use std::collections::HashMap;

  pub fn print(num: u32) {
    let my_vec = vec![num, 2, 3];

    let mut norm_vec = Vec::new();
    norm_vec.push(num);

    println!("{:?} {:?}", my_vec, norm_vec);

    let mut numbers = HashMap::new();
    numbers.insert(String::from("One"), 1);
    numbers.insert(String::from("Two"), 2);

    println!("{:?}", numbers);
  }
}

fn main() {
  counts::print(16);
}
