use std::io::{self, BufRead};

fn evenfibonaccisum (n: u64, sum: u64) -> Vec<u64> {
  println!("{}: {}", n, sum);

  match n {
    0 => vec![1, sum],
    1 => vec![1, sum],
    _ => {
      let fnum1 = evenfibonaccisum(n - 1, sum);
      let fnum2 = evenfibonaccisum(n - 2, sum);

      let fnum: u64 = fnum1[0] + fnum2[0];
      let mut fsum: u64 = fnum1[1] + fnum2[1];
      if fnum % 2 == 0 {
        fsum += fnum;
      }
      vec![fnum, fsum]
    },
  }
}

fn main() {
  let stdin = io::stdin();

  let mut tests_num = String::new();
  stdin.read_line(&mut tests_num).expect("failed to read stdin");
  let tests_num: u32 = tests_num.trim().parse().expect("invalid input");

  let mut i: u32 = 0;

  while i < tests_num {
    let mut num = String::new();
    stdin.read_line(&mut num).expect("failed to read stdin");
    let num: u64 = num.trim().parse().expect("invalid input");

    let res = evenfibonaccisum(num, 0);
    println!("result = {}", res[1]);

    i += 1;
  }
}
