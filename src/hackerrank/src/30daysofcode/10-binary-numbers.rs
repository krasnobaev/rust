use std::io;

fn main() {
  let stdin = io::stdin();

  let mut num = String::new();
  stdin.read_line(&mut num).expect("failed to read stdin");
  let mut num: u32 = num.trim().parse().expect("invalid input");
  let mut ones: u16 = 0;
  let mut maxseq: u16 = 0;

  while num != 0 {
    if num / 2 * 2 != num {
      ones += 1;
    } else if ones > maxseq {
      maxseq = ones;
      ones = 0;
    } else {
      ones = 0;
    }

    num /= 2;
  }

  if ones > maxseq {
    maxseq = ones;
  }

  println!("{}", maxseq);
}
