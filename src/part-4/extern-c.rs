extern "C" {
  fn printf(text: &str);
}

fn main() {
  unsafe {
    printf("Hello");
  };
}

