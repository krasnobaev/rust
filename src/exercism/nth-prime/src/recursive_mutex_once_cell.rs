use std::sync::Mutex;
use once_cell::sync::Lazy;

const PRIME_MAX: usize = 10_000;
static PRIMES: Lazy<Mutex<[u32;PRIME_MAX+1]>> = Lazy::new(|| {
    let mut primes = [0;PRIME_MAX+1];
    primes[0] = 1;
    Mutex::new(primes)
});

fn next_prime (n: usize, primes: &mut [u32;PRIME_MAX+1]) -> u32 {
    // println!("next_prime({})", n);

    match n {
        0 => 2,
        _ => {
            let prev_prime = match primes[n-1] {
                0 => next_prime(n-1, primes),
                _ => primes[n-1],
            };
            // println!("next_prime(): prev prime ({}) was {}", n-1, prev_prime);

            let mut candidate = prev_prime;
            'is_not_prime: loop {
                candidate += 2; // except 1st, primes are odd

                let mut is_not_divisible = true;
                'is_not_divisible: for i in 1..n {
                    // println!("next_prime(): assert divisibility {} % {} === {}", candidate, PRIMES[i], candidate % PRIMES[i]);
                    if candidate % primes[i] == 0 {
                        // println!("next_prime(): {} is divisible by {}", candidate, PRIMES[i]);
                        is_not_divisible = false;
                        break 'is_not_divisible;
                    }
                }

                if is_not_divisible {
                    break 'is_not_prime;
                }
            }
            primes[n] = candidate;
            primes[n]
        }
    }
}

pub fn nth(n: u32) -> u32 {
    println!("nth({})", n);
    let n: usize = n as usize;

    let primes_lock = &mut PRIMES.lock().unwrap();
    next_prime(n, primes_lock)
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
