static mut WELCOME_TEXT: &str = "Hello, Citizens!";
static mut TIMER: u32 = 44;

fn add_time(inc: u32) {
  unsafe {
    TIMER += inc;
  }
}

fn main() {
  add_time(69 - 1);

  unsafe {
    println!("Timer is at {}", TIMER);
  }
}

