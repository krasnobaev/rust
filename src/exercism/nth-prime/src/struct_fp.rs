struct Prime {
    primes: Vec<u32>,
    count: usize,
}

impl Prime {
    pub fn new () -> Self {
        Self { primes: vec![], count: 0 }
    }

    fn is_prime (self: &Self, candidate: u32) -> bool {
        // let candidate_sqrt = (candidate as f32).sqrt() as u32;
        // !self.primes.iter()
        //     .filter(|&&x| x <= candidate_sqrt)
        //     .any(|x| candidate % x == 0)
        let m = (candidate as f32).sqrt() as u32;
        !(2..m+1).any(|x| candidate % x == 0)
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = match self.count {
            0 => 2,
            1 => 3,
            _ => (self.primes[self.count - 1] + 2..)
                    .step_by(2)
                    .filter(|&candidate| self.is_prime(candidate))
                    .nth(0)
                    .unwrap(),
        };

        self.primes.push(prime);
        self.count += 1;
        Some(prime)
    }
}

pub fn nth(n: u32) -> u32 {
    Prime::new()
        .nth(n as usize)
        .unwrap()
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

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}
