pub mod math {
    // sum starts 1, because 1 is counted.
    pub fn sum_divisors(x: i32) -> i32 {
        let mut sum: i32;
        // root of x.
        let mut rt = (x as f32)
            .sqrt() as i32;
        
        (sum, rt) = match rt*rt {
            n if n == x => (1 + rt, rt-1),
            _ => (1, rt),
        };

        // f: factor.
        let (mut f, step) = match x & 1 {
            0 => (2, 1),
            _ => (3, 2),
        };

        while f <= rt {
            if x % f == 0 {
                sum += f + x / f;
            }
            f += step;
        }    
        sum
    }
    // find all primes up to n.
    pub fn find_primes(n: usize) -> Vec<usize> {

        let mut is_prime = vec![true; n * 2];
        let mut p = 2;
        let mut i: usize;
        // sieve of erathosenes.
        while p * p <= n {
            if is_prime[p] {

                i = p * p;
                while i <= n {
                    is_prime[i] = false;
                    i += p;
                }
            }
            p += 1;
        }
        // pre allocate sqrt of n.
        let x = (n as f64).sqrt() as usize;
        let mut primes = Vec::with_capacity(x);

        for i in 2..is_prime.len() {
            if is_prime[i] {
                primes.push(i);
            }
        }
        primes
    }
    // check if n is prime. only valid for 
    // n larger than 3.
    pub fn is_prime(n: usize) -> bool {
        if n & 1 == 0 {
            return false;
        }
        if n % 3 == 0 {
            return false;
        }

        let r = (n as f64).sqrt() as usize;
        let mut f = 5;

        while f <= r {
            if n % f == 0 {
                return false;
            }
            if n % (f + 2) == 0 {
                return false;
            }
            f += 6;
        }
        true
    }
}
