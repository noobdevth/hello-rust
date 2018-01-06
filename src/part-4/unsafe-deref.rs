fn deref_rawptr(mut num: u64) {
  let r9 = &num as *const u64;
  let r10 = &mut num as *mut u64;

  unsafe {
    *r10 = 112;

    println!("R9 is {}", *r9);
    println!("R10 is {}", *r10);
  }
}

fn main() {
  let num: u64 = 44;

  deref_rawptr(num);
}
