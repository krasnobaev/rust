fn fast_part_pow(x: u64, y: u64) -> u64 {
  (1..y).fold(x, |acc, _i| (acc*x) % 1000)
}

fn _old_part_pow(x: u64, y: u32) -> u64 {
  println!("part_pow {}^{}≈…", x, y);//part_pow(*x, acc as usize));

  match y {
    0 => 1,
    1 => x,
    _ => {
      match x {
        0|1 => x,
        x if x%10 == 1 => {
          ((x as u128 % 1000).pow(y % 10) % 1000) as u64
        },
        x if x%10 == 0 => 100,
        x if x%10 == 2 => match y {
          _ => ((x as u128 % 100).pow(((y - 2) % 20) + 2) % 100) as u64,
        },
        x if x%10 == 3 => {
          ((x as u128 % 100).pow(y % 20) % 100) as u64
        },
        x if x%10 == 4 => match y {
          _ => ((x as u128 % 100).pow(y % 11) % 100) as u64,
        },
        x if x%10 == 5 => if y == 1 { 5 } else { 25 },
        x if x%10 == 6 => match y {
          2 => 36,
          3 => 16,
          4 => 96,
          5 => 76,
          _ => { (x%100).pow((y%5)+5) },
        },
        x if x%10 == 7 => match y {
          _ => (x%10).pow(y % 4) as u64,
        },
        x if x%10 == 8 => match y {
          _ => ((x as u128 % 100).pow((y-1)%20 as u32 + 1) % 100) as u64,
        },
        x if x%10 == 9 => match y {
          _ => ((x as u128 % 100).pow(y % 10) % 100) as u64,
        },
        _ => ((x as u128 % 100).pow(y % 100) % 1000) as u64,
      }
    }
  }
}

fn part_pow(x: u64, y: u64) -> u64 {
  print!("part_pow {}^{}≈…", x, y);

  let prod = match y {
    0 => 1,
    1 => x,
    _ => match x {
      0 => 0,
      x if x%10 == 0 => 100,
      _ => {
        (1..y).fold(x, |acc, _i| (acc*x) % 1000)
      }
    },
  };

  println!("{}", prod);
  prod
}

fn _last_digit(lst: &[u64]) -> u64 {
  println!("trying {:?}", lst);

  let mut list: Vec<u64> = Vec::from(lst);
  let last = list.pop().unwrap_or(1);
  list.iter().rev().fold(last, |acc, x| {
    // part_pow(*x%1000, acc as u32)
    part_pow(*x, acc)
  }) % 10
}

fn last_digit(lst: &[u64]) -> u64 {
  println!("trying {:?}", lst);

  Vec::from(lst).iter().rev().fold(1, |y, x| {
    match y {
      0 => 1,
      1 => *x,
      _ => match x {
        0|1 => *x,
        x if *x%10 == 0 => 100,
        x if *x%10 == 5 => if y == 1 { 5 } else { 25 },
        x if *x%10 == 6 => match y {
          2 => 36,
          3 => 16,
          4 => 96,
          5 => 76,
          _ => part_pow(*x, (y%5)+5),
        },
        x if *x%10 == 7 => part_pow(*x, y%20),
        _ => part_pow(*x, y)
      },
    }
  }) % 10
}

fn main() { last_digit(&[]); }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_tests() {
      let tests = vec![
          (vec![3,4,2], 1),
          (vec![], 1),
          (vec![0, 0], 1),
          (vec![0, 0, 0], 0),
          (vec![1, 2], 1),
          (vec![3, 4, 5], 1),
          (vec![4, 3, 6], 4),
          (vec![7, 6, 21], 1),
          (vec![12, 30, 21], 6),
          (vec![2, 2, 2, 0], 4),
          (vec![101, 2], 1),
          (vec![2, 101, 2], 2),
          (vec![2, 2, 101, 2], 6),
          (vec![2, 0, 0, 2, 0, 2, 2, 0, 1, 2], 2),
          (vec![937640, 767456, 981242], 0),
          (vec![123232, 694022, 140249], 6),
          (vec![499942, 898102, 846073], 6),
          (vec![82242, 254719, 736371], 8),
          (vec![625703, 43898, 614961, 448629], 1),

          (vec![279672, 922485, 566145, 278171, 698519, 823278, 510250, 663175, 625435, 297958], 2),
          (vec![996654, 580411, 639144, 852202, 700575, 854968, 252528, 630604, 152761, 591422], 4),
          (vec![296637, 250682, 794826, 707937, 201598, 630473, 812893, 580876, 638324, 248064], 1),
          (vec![656217, 961760, 952119, 21677, 914419, 643453, 94408, 767065, 391721, 218326], 1),
      ];

      for test in tests {
          assert_eq!(last_digit(&test.0), test.1);
      }
  }

  #[test]
  fn seq2_tests() {
    for i in 1..128 {
      assert_eq!(part_pow(2, i)%100, ((2 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..36 {
      assert_eq!(part_pow(12, i)%100, ((12 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..20 {
      assert_eq!(part_pow(22, i)%100, ((22 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..20 {
      assert_eq!(part_pow(32, i)%100, ((32 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..20 {
      assert_eq!(part_pow(42, i)%100, ((42 as u128).pow(i as u32) % 100) as u64);
    }
  }

  #[test]
  fn seq3_tests() {
    for i in 1..81 {
      assert_eq!(part_pow(3, i)%100, ((3 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..35 {
      assert_eq!(part_pow(13, i)%100, ((13 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..29 {
      assert_eq!(part_pow(23, i)%100, ((23 as u128).pow(i as u32) % 100) as u64);
    }
  }

  #[test]
  fn seq7_tests() {
    for i in 1..41 {
      assert_eq!(part_pow(7, i)%100, fast_part_pow(7, i)%100);
    }

    for i in 1..20 {
      assert_eq!(part_pow(17, i)%100, fast_part_pow(17, i)%100);
    }

    for i in 1..27 {
      assert_eq!(part_pow(27, i)%100, fast_part_pow(27, i)%100);
    }

    for i in 1..21 {
      assert_eq!(part_pow(37, i)%100, fast_part_pow(37, i)%100);
    }

    for i in 1..20 {
      assert_eq!(part_pow(97, i)%100, fast_part_pow(97, i)%100);
    }
  }

  #[test]
  fn seq8_tests() {
    for i in 1..41 {
      assert_eq!(part_pow(8, i)%100, ((8 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..20 {
      assert_eq!(part_pow(18, i)%100, ((18 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..27 {
      assert_eq!(part_pow(28, i)%100, ((28 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..21 {
      assert_eq!(part_pow(38, i)%100, ((38 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..20 {
      assert_eq!(part_pow(98, i)%100, ((98 as u128).pow(i as u32) % 100) as u64);
    }
  }

  #[test]
  fn seq9_tests() {
    for i in 1..41 {
      assert_eq!(part_pow(9, i)%100, ((9 as u128).pow(i as u32) % 100) as u64);
    }

    for i in 1..31 {
      assert_eq!(part_pow(19, i)%100, ((19 as u128).pow(i as u32) % 100) as u64);
    }
  }

  #[test]
  fn seq10000s_tests() {
    assert_eq!(part_pow(2, 100)%100, 76);
    assert_eq!(part_pow(2, 1000)%100, 76);
    assert_eq!(part_pow(2, 10000)%100, 76);
    assert_eq!(part_pow(3, 100), 1);
    assert_eq!(part_pow(3, 1000), 1);
    assert_eq!(part_pow(3, 1000), 1);
  }
}
