// https://exercism.org/tracks/rust/exercises/nth-prime/solutions/plippe

pub fn is_prime(n: u32) -> bool {
    ! (2..n - 1).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    match n {
        n if n <= 0 => 0,
        n => (1..).filter(|c| is_prime(*c)).nth(n as usize).unwrap(),
    }
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(1 - 1), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(2 - 1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(6 - 1), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_001 - 1), 104_743);
}

// additional tests

// #[test]
// fn test_20k_prime() {
//     assert_eq!(nth(20_001 - 1), 611957);
// }

// #[test]
// fn test_30k_prime() {
//     assert_eq!(nth(30_001 - 1), 611957);
// }

// #[test]
// fn test_40k_prime() {
//     assert_eq!(nth(40_001 - 1), 611957);
// }

// #[test]
// fn test_50k_prime() {
//     assert_eq!(nth(50_001 - 1), 611957);
// }

// #[test]
// fn test_100k_prime() {
//     assert_eq!(nth(100_001 - 1), 1_299_721);
// }
