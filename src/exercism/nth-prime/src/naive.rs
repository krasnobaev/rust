const SIEVE_MAX: usize = 100_000_000;
const INDEX_MAX: usize = 100_000;

struct Prime {
    sieve: [bool; SIEVE_MAX],
    // sieve: Vec<bool>,
    // sieve: std::collections::HashMap<usize, bool>,
    index: [usize; INDEX_MAX],
}
impl Prime {
    pub fn new() -> Self {
        Self {
            sieve: [true; SIEVE_MAX],
            // sieve: vec![true; SIEVE_MAX],
            // sieve: std::collections::HashMap::new(),
            index: [0; INDEX_MAX],
        }
    }
}

// thread_local! (static PRIME_MEM: Prime = Prime::new());
pub fn nth(n: u32) -> u32 {
    println!("n: {}", n);

    let mut prime = Prime::new();
    // PRIME_MEM.with (|prime: &mut Prime| {
    let n: usize = n as usize;
    // let mut i: usize = 2;
    prime.index[1] = 2;
    let mut prime_ind = 1;
    let n_max = if (n*n) < SIEVE_MAX { (n+10)^(n*n*n) } else { return 0 };
    let n_max = if n_max > SIEVE_MAX { SIEVE_MAX } else { n_max };

    for i in 2..n_max {
        if prime.sieve[i] == true {
            prime.index[prime_ind] = i;
            prime_ind += 1;
            for j in ((i*i)..n_max).step_by(i) {
                prime.sieve[j] = false;
            }
        }
        // if !prime.sieve.contains_key(&i) {
        //     prime.index[prime_ind] = i;
        //     prime_ind += 1;
        //     ((i*i)..n_max).step_by(i).for_each(|v| {
        //         prime.sieve.insert(v, false);
        //     });
        // }

        if i*i > n_max { break; }
    }

    println!("primes: {:?}", prime.index[600..700].iter());
    // println!("some prime: {}", prime.index[500]);
    prime.index[n+1] as u32
    // })
}
// memoization - https://stackoverflow.com/questions/22752775/

fn main() {
    const N: u32 = 400;
    println!("{}th prime is {}", N, nth(N));
}

// #[test]
// fn test_first_prime() {
//     assert_eq!(nth(0), 2);
// }

// #[test]
// fn test_second_prime() {
//     assert_eq!(nth(1), 3);
// }

// #[test]
// fn test_sixth_prime() {
//     assert_eq!(nth(5), 13);
// }

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}
