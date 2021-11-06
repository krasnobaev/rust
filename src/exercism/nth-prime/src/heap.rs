fn is_divisible (candidate: u32, n: usize, primes: &Vec<u32>) -> bool {
    for i in 1..n {
        if primes[i] == 0 { continue }
        if primes[i] > (candidate as f32).sqrt() as u32 { continue }
        if candidate % primes[i] == 0 { return true }
    }

    false
}

fn get_next_prime (i: usize, primes: &Vec<u32>) -> u32 {
    let mut candidate = primes[i-1] + 2;
    while is_divisible(candidate, primes.len(), primes) {
        candidate += 2;
    }
    candidate
}

fn compute_primes (from: usize, n: usize, primes: &mut Vec<u32>) {
    for i in from..=n {
        primes.push(get_next_prime(i, primes));
    }
}

pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;
    let mut primes = vec![2,3];
    let highest_stored_prime = primes.len();

    if highest_stored_prime < n {
        compute_primes(highest_stored_prime, n, &mut primes);
    }

    primes[n]
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
