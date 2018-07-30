use std::io;

fn main() {
  let stdin = io::stdin();

  let mut n = String::new();
  stdin.read_line(&mut n).expect("failed to read stdin");
  let n: i32 = n.trim().parse().expect("invalid input");

  let mut res: i32;
  let mut i: i32 = 1;

  while i <= 10 {
    res = n * i;
    println!("{} x {} = {}", n, i, res);

    i += 1;
  }
}
