use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut entry1 = String::new();
  stdin.read_line(&mut entry1).expect("failed to read stdin");
  let mut entry1: String = entry1.trim().parse().expect("invalid input");
  let returned: Vec<&str> = entry1.split(' ').collect();
  let ret_day: i32 = returned[0].parse::<i32>().unwrap();
  let ret_mon: i32 = returned[1].parse::<i32>().unwrap();
  let ret_yer: i32 = returned[2].parse::<i32>().unwrap();

  let mut entry2 = String::new();
  stdin.read_line(&mut entry2).expect("failed to read stdin");
  let mut entry2: String = entry2.trim().parse().expect("invalid input");
  let expected: Vec<&str> = entry2.split(' ').collect();
  let exp_day: i32 = expected[0].parse::<i32>().unwrap();
  let exp_mon: i32 = expected[1].parse::<i32>().unwrap();
  let exp_yer: i32 = expected[2].parse::<i32>().unwrap();

  let mut fine: i32;
  if ret_yer > exp_yer {
    fine = 10000;
  } else if ret_yer < exp_yer {
    fine = 0;
  } else if ret_mon > exp_mon {
    fine = 500 * (ret_mon - exp_mon);
  } else if ret_mon < exp_mon {
    fine = 0;
  } else if ret_day - exp_day > 1 {
    fine = 15 * (ret_day - exp_day);
  } else {
    fine = 0;
  }

  println!("{}", fine);
}
