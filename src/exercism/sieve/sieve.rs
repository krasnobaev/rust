struct Prime {
    pub primes: Vec<u64>,
}

impl Prime {
    pub fn new () -> Self {
        Self { primes: vec![] }
    }

    fn is_prime (self: &Self, candidate: u64) -> bool {
        let m = (candidate as f32).sqrt() as u64;
        !(2..m+1).any(|x| candidate % x == 0)
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let prime = match self.primes.len() {
            0 => 2,
            1 => 3,
            _ => (self.primes.last().unwrap() + 2 ..)
                    .step_by(2)
                    .filter(|&candidate| self.is_prime(candidate))
                    .nth(0)
                    .unwrap(),
        };

        self.primes.push(prime);
        Some(prime)
    }
}

pub fn primes_up_to(n: u64) -> Vec<u64> {
    Prime::new()
        .take(n as usize)
        .filter(|&x| x <= n)
        .collect()
}
