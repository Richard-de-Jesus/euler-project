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
}
