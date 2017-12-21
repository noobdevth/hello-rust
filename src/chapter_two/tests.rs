use std::ops;

fn plus<T: ops::Add<Output = T>>(x: T, y: T) -> T {
  x + y
}

fn minus<T: ops::Sub<Output = T>>(x: T, y: T) -> T {
  x - y
}

#[cfg(test)]
mod big_shaq {
  use super::*;

  #[test]
  fn two_plus_two_is_four() {
    assert_eq!(plus(2, 2), 4);
  }

  #[test]
  fn minus_one_thats_three_quick_maths() {
    assert_eq!(minus(4, 1), 3);
  }
}
