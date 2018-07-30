use std::io;

fn main() {
  let stdin = io::stdin();

  let mut n = String::new();
  stdin.read_line(&mut n).expect("failed to read stdin");
  let n: i32 = n.trim().parse().expect("invalid input");

  if n & 1 == 1 {
    println!("Weird");
  } else if n > 20 {
    println!("Not Weird");
  } else if n > 5 {
    println!("Weird");
  } else if n > 2 {
    println!("Not Weird");
  }
}
