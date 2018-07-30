use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut array_size = String::new();
  stdin.read_line(&mut array_size).expect("failed to read stdin");
  // let array_size: i32 = array_size.trim().parse().expect("invalid input");

  let numbers: Vec<i32> =
    stdin.lock()
      .lines().next().unwrap().unwrap()
      .split(' ').rev().map(|s| s.trim())
      .filter(|s| !s.is_empty())
      .map(|s| s.parse().unwrap())
      .collect();

  let numbers =
    numbers.iter()
      .fold(String::new(), |acc, &num| acc + &num.to_string() + " ");

  println!("{}", numbers);

}
