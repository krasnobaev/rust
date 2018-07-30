use std::io;

fn main() {
  let stdin = io::stdin();

  let mut tests_num = String::new();
  stdin.read_line(&mut tests_num).expect("failed to read stdin");
  let tests_num: i32 = tests_num.trim().parse().expect("invalid input");

  let mut i: i32 = 0;

  while i < tests_num {
    let mut line = String::new();
    stdin.read_line(&mut line).expect("failed to read stdin");
    let line: String = line.trim().parse().expect("invalid input");

    let mut evenline = String::new();
    let mut oddline = String::new();

    for (j, c) in line.chars().enumerate() {
      if j & 1 == 1 {
        oddline.push(c);
      } else {
        evenline.push(c);
      }
    }

    println!("{} {}", evenline, oddline);

    i += 1;
  }
}
