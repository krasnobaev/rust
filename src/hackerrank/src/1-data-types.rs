use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut int_num = String::new();
  stdin.read_line(&mut int_num).expect("failed to read stdin");
  let int_num: i32 = int_num.trim().parse().expect("invalid input");

  let mut double_num = String::new();
  stdin.read_line(&mut double_num).expect("failed to read stdin");
  let double_num: f32 = double_num.trim().parse().expect("invalid input");

  let line = stdin.lock()
      .lines()
      .next()
      .expect("there was no next line")
      .expect("the line could not be read");

  println!("{:.0}", int_num + 4);
  println!("{:.1}", double_num + 4.0);
  println!("HackerRank {}", line);
}
