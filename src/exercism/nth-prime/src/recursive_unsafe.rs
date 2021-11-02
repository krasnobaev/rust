// https://cp-algorithms.com/algebra/sieve-of-eratosthenes.html
// https://cp-algorithms.com/algebra/prime-sieve-linear.html
// https://stackoverflow.com/questions/39840663/recursive-function-calculating-factorials-leads-to-stack-overflow
// https://stackoverflow.com/questions/2630054/does-c-limit-recursion-depth

const PRIME_MAX: usize = 260_000; // calculating 270_000th Prime causes stack overflow while calling from main
// const PRIME_MAX: usize = 10_000; // #[test]s also may cause stack overflow after 10_000
static mut PRIMES: [u32;PRIME_MAX+1] = [0;PRIME_MAX+1];

fn next_prime (n: usize) -> u32 {
    // println!("next_prime({})", n);

    match n {
        0 => 2,
        _ => unsafe {
            let mut prev_prime = match PRIMES[n-1] {
                0 => next_prime(n-1),
                _ => PRIMES[n-1],
            };
            // println!("next_prime(): prev prime ({}) was {}", n-1, prev_prime);

            'is_not_prime: loop {
                prev_prime += 2; // except 1st, primes are odd

                let mut is_not_divisible = true;
                'is_not_divisible: for i in 1..n {
                    // println!("next_prime(): assert divisibility {} % {} === {}", prev_prime, PRIMES[i], prev_prime % PRIMES[i]);
                    if prev_prime % PRIMES[i] == 0 {
                        // println!("next_prime(): {} is divisible by {}", prev_prime, PRIMES[i]);
                        is_not_divisible = false;
                        break 'is_not_divisible;
                    }
                }

                if is_not_divisible {
                    break 'is_not_prime;
                }
            }
            PRIMES[n] = prev_prime;
            PRIMES[n]
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;
    unsafe { PRIMES[0] = 1; }
    next_prime(n)
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
