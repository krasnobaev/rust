use std::sync::Mutex;
use once_cell::sync::Lazy;

// const PRIME_MAX: usize = 2_094_148; // stack overflow on 2_094_149
const PRIME_MAX: usize = 100_000; // stack overflow on 2_094_149

static PRIMES: Lazy<Mutex<[u32;PRIME_MAX+1]>> = Lazy::new(|| {
    let mut primes = [0;PRIME_MAX+1];
    primes[0] = 2;
    primes[1] = 3;
    Mutex::new(primes)
});

fn is_divisible (candidate: u32, n: usize, primes: &mut [u32;PRIME_MAX+1]) -> bool {
    for i in 1..=n {
        if primes[i] == 0 { continue }
        if primes[i] > (candidate as f32).sqrt() as u32 { continue }
        if candidate % primes[i] == 0 { return true }
    }

    false
}

fn get_next_prime (i: usize, n: usize, primes: &mut [u32;PRIME_MAX+1]) -> u32 {
    let mut candidate = primes[i-1];
    loop {
        candidate += 2;
        if !is_divisible(candidate, n, primes) { break }
    }
    candidate
}

fn compute_primes (from: usize, n: usize, primes: &mut [u32;PRIME_MAX+1]) {
    for i in from..=n {
        if primes[i] == 0 {
            primes[i] = get_next_prime(i, n, primes);
        };
    }
}

pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;

    let mut primes_lock = PRIMES.lock().unwrap();
    if let Some(highest_stored_prime) = primes_lock.iter().position(|&x| x == 0) {
        if highest_stored_prime < n {
            compute_primes(highest_stored_prime, n, &mut primes_lock);
        }
    }

    primes_lock[n]
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
