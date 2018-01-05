extern crate greprs;

use greprs::Config;
use std::env;
use std::io::Write;
use std::process;

fn main() {
  let mut stderr = std::io::stderr();

  let config = Config::new(env::args()).unwrap_or_else(|err| {
    let _ = writeln!(&mut stderr, "Problem parsing arguments: {}", err);
    process::exit(1);
  });

  if let Err(err) = greprs::run(config) {
    let _ = writeln!(&mut stderr, "Failure: {}", err);
    process::exit(1);
  };
}
