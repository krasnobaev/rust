struct Prime {
    primes: Vec<u32>,
    count: usize,
}

impl Prime {
    pub fn new () -> Self {
        Self { primes: vec![], count: 0 }
    }

    fn is_divisible (self: &Self, candidate: u32) -> bool {
        for i in 1..self.primes.len() {
            if self.primes[i] > (candidate as f32).sqrt() as u32 { continue }
            if candidate % self.primes[i] == 0 { return true }
        }
        return false
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = match self.count {
            0 => 2,
            1 => 3,
            _ => {
                let mut candidate = self.primes[self.count - 1] + 2;
                while self.is_divisible(candidate) {
                    candidate += 2;
                }
                candidate
            },
        };

        self.primes.push(prime);
        self.count += 1;
        Some(prime)
    }
}

pub fn nth(n: u32) -> u32 {
    Prime::new()
        .into_iter()
        .take(n as usize + 1)
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
