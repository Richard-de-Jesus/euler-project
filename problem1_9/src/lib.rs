use std::time::Instant;
use library::math;
use std::hint::black_box;
// find the sum of all multiples of 3 or 5 
// that are below 1000.
// 233168 
pub fn _1() {
    let start = Instant::now();

    let num = black_box(0..1000)
        .filter(|i| i % 3 == 0 || i % 5 == 0)
        .sum::<u32>();

    println!("answer 1: {num}. T: {:?}", start.elapsed());
}
// find the sum of all even fibonacci terms.
// that are under 4 million.
// 4613732
pub fn _2() {
    let start = Instant::now();
    let mut num = 0;

    let mut x: i32;
    let (mut y, mut z) = (1, 2);



    while z < 4_000_000 {
         // checking if term is even;
        if z & 1 == 0 {
            num += z;
        }

        x = y;
        y = z;
        z = x + y;
    }
    println!("answer 2: {num}. T: {:?}", start.elapsed());
}
// find the largest prime factor of num.
// 6857
pub fn _3() {
    let start = Instant::now();
    let mut num: i64 = 600_851_475_143;

    let mut factor = 2;
    let mut prime = 0;

    while factor <= num {
        match num % factor {
            0 => {
                prime = factor;
                num /= factor;
            },
            _ => factor += 1,
        };
    }
    println!("answer 3: {prime}. T: {:?}", start.elapsed());
}
// find largest palindrome made from a product
// of 2 3-digit numbers.
// 906609
pub fn _4() {
    let start = Instant::now();
    let mut max = 0u64;


    let is_palindrome = |mut x| {
        let mut rev_x = 0;
        let initial_x = x;

        while x > 0 {
            rev_x = 10 * rev_x + x % 10;
            x /= 10;
        }
        rev_x == initial_x
    };

    for i in (100..=999).rev() {
        for j in (i..=999).rev() {
            let num = i * j;

            if num <= max {
                break;
            }

            if is_palindrome(num) {
                max = num;
            }
        }
    }
    println!("answer 4: {max}. T: {:?}", start.elapsed());
}
// find smallest number that can be divided 
// by 1 to 20 without remainder.
// 232792560
#[allow(unused)]
pub fn _5() {
    let start = Instant::now();
    // largest divisor.
    let k = black_box(20usize);
    // all primes up to k.
    let primes = math::find_primes(k);
    // expoents.
    let mut exp = vec![0u32; primes.len()];

    let limit = (k as f64).sqrt() as usize;
    
    let log2k = f64::log2(k as f64);
    let evaluate = |e: &mut Vec<u32>, i| {
        let p = primes[i];

        if p <= limit {
            e[i] = (log2k / f64::log2(p as f64)) as u32;
            return true;
        }
        false
    };
    let mut num = 1;

    let mut idx = 0;
    let mut check = true;

    while primes[idx] <= k {
        exp[idx] = 1;

        if check {
            check = evaluate(&mut exp, idx);
        }
        num *= primes[idx].pow(exp[idx]); 
        idx += 1;
    }

    let x = start.elapsed();
    println!("answer 5: {num}. T: {x:?}");
}
// Find the difference between the sum 
// of the squares of the first 100 natural 
// numbers and the square of the sum.
// 25164150
pub fn _6() {
    let start = Instant::now();
    // sum of squares. x² + y² + z²... 
    let mut sum_sq = 0;
    // square of the sum. (x + y + z ...)²
    let mut sq_sum = 0;

    for i in 1..=100 {
        sum_sq += i * i;

        sq_sum += i; 
    }
    sq_sum *= sq_sum;

    let answer = sq_sum - sum_sq;
    let x = start.elapsed();
    println!("answer 6: {answer}. T: {x:?}");

}
// what is the 10_001th prime?
// 104743
pub fn _7() {
    let start = Instant::now();
 
    // counter starts at 4,
    // cause n starts at 4th prime.
    let mut counter = 4;
    let mut n = 7;

    while counter != 10_001 {
        n += 2;
        
        if math::is_prime(n) {
            counter += 1;
        }
    }
    let x = start.elapsed();
    println!("answer 7: {n}. T: {x:?}");    
}
// take the  13 adjacents digits in the 1000 
// digit number that have the greatest product.
// find that product.
// 23514624000
pub fn _8() {
    let start = Instant::now();
    let mut num = include_bytes!("1000_digit.txt")
        .to_vec();
    // - 1 to remove newline at end. 
    let len = num.len() - 1;
    // get numerical value.
    (0..len).for_each(|i| num[i] -= b'0');

    let mut max = 0;
    let mut temp: u64;





    // -12 because we iterate 13 numbers 
    // at a time, to not go out of bounds.
    for i in 0..num.len() - 12 {
        temp = 1; 

        // multiply 13 adjacents digits and store them. 
         (i..=i+12)
            .for_each(|j| temp *= num[j] as u64);

        if temp > max {
            max = temp;
        }
    }
    let x = start.elapsed();
    println!("answer 8: {max}. T: {x:?}");
}

// There exists exactly one Pythagorean 
// triplet for which a + b + c = 1000.
// find the product abc.
// 31875000
pub fn _9() {
    let start = Instant::now();
    // the a+b+c number.
    let s = black_box(1000);

    let mut num = 0;

    'x: for a in 1..400 {
        for b in a + 1..500 {

            let c = s - a - b;

            if a*a + b*b == c*c {
                num = a*b*c;
                break 'x;
            }
        }
    }
    let x = start.elapsed();
    println!("test: {num}. T: {x:?}");
}
