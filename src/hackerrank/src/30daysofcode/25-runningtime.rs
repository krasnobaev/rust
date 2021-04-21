use std::io::{self, BufRead};

fn isprime (n: u32) -> bool {
  match n {
    0 | 1 => false,
    2 => true,
    _even if n % 2 == 0 => false,
    _ => {
        let sqrt_limit = (n as f32).sqrt() as u32;
        (3..=sqrt_limit).step_by(2).find(|i| n % i == 0).is_none()
    }
  }
}

fn main() {
  let stdin = io::stdin();

  let mut tests_num = String::new();
  stdin.read_line(&mut tests_num).expect("failed to read stdin");
  let tests_num: u8 = tests_num.trim().parse().expect("invalid input");

  let mut i: u8 = 0;

  while i < tests_num {
    let mut num = String::new();
    stdin.read_line(&mut num).expect("failed to read stdin");
    let num: u32 = num.trim().parse().expect("invalid input");

    if isprime(num) {
      println!("Prime");
    } else {
      println!("Not prime");
    }

    i += 1;
  }
}
