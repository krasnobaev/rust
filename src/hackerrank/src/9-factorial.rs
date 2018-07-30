use std::io;

fn factorial (n: u32) -> u32 {
  match n {
    0 => 1,
    _ => n * factorial(n-1)
  }
}

fn main() {
  let stdin = io::stdin();

  let mut init_value = String::new();
  stdin.read_line(&mut init_value).expect("failed to read stdin");
  let init_value: u32 = init_value.trim().parse().expect("invalid input");

  let final_value: u32 = factorial(init_value);

  println!("{}", final_value);
}
