// https://exercism.org/tracks/rust/exercises/nth-prime/solutions/plippe

pub fn is_prime(n: u32) -> bool {
    let m = (n as f32).sqrt() as u32;
    let is_divisor = |x| n % x == 0;

    !(2..m+1).any(is_divisor)
}

pub fn nth(n: u32) -> u32 {
    match n {
        n if n <= 0 => 0,
        n => (1..).filter(|c| is_prime(*c)).nth(n as usize).unwrap(),
    }
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(1), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(2), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(6), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_001), 104_743);
}
