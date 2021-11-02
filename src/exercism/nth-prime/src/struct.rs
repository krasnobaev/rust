const PRIME_MAX: usize = 100_000;

// fn is_prime(n: u32, d: u32) -> bool {
//     match (n,d) {
//         (n,_) if n < 2 => false,
//         (_,d) if d == 1 => true,
//         (n,d) if n % d == 0 => false,
//         (_,_) => is_prime(n, d-1),
//     }
// }

// #[derive(Copy,Clone)]
struct Prime {
    primes: [u32; PRIME_MAX],
}
impl Prime {
    pub fn new () -> Self {
        let mut primes = [0; PRIME_MAX];
        primes[0] = 1;
        primes[1] = 2;

        Self {
            primes,
        }
    }

    pub fn calc_nth (self: &mut Self, n: usize) -> &mut Self {
        println!("nth({})", n);
        let mut prev_prime = self.primes[n - 1];
        if prev_prime == 0 {
            prev_prime = self.calc_nth(n-1).get_nth(n-1);
            println!("{}th prime is {}", n-1, prev_prime);
        }

        'is_not_prime: loop {
            prev_prime += 2;

            let mut is_not_divisible: bool = true;
            // 'divisible: for i in 1..n {
            for i in 1..n {
                if prev_prime % self.primes[i] == 0 {
                    is_not_divisible = false;
                    break 'is_not_prime;
                }
            }
        }

        self.primes[n] = prev_prime;
        self // self.primes[n]
    }

    pub fn print_primes (self: Self, from: usize, to: usize) -> Self {
        print!("({}..{}): ", from, to);
        for i in from..=to {
            print!("{} ", self.primes[i]);
        }
        println!("");

        self
    }

    pub fn get_nth (self: Self, n: usize) -> u32 {
        self.primes[n]
    }
}

pub fn nth(n: u32) -> u32 {
    let n: usize = n as usize;
    Prime::new().calc_nth(n).print_primes(0, n+1).get_nth(n)
}

fn main() {
    const N: u32 = 5;
    println!("RESULT: {}th prime is {}", N, nth(N));
}

#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

// #[test]
// fn test_big_prime() {
//     assert_eq!(nth(10_000), 104_743);
// }
