use std::io;

fn main() {
  let stdin = io::stdin();

  println!("Hello, World.");

  for line in stdin.lock().lines() {
    println!("{}", line.unwrap());
  }
}
