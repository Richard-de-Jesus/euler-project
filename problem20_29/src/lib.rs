use std::time::Instant;
use library::*;
use bigdecimal::BigDecimal;
use bigdecimal::num_bigint::BigUint;

pub mod lib1;
pub use lib1::*;
// Find the sum of the digits of 100 factorial.
// 648
pub fn _20() {
    let start = Instant::now();
    let mut n =  BigUint::from(2u32);

    for i in 3..=100u32 {
        n *= i;
    }

    let mut sum = 0;
    let digits = n.to_str_radix(10);
    for dig in digits.chars() {

        let x = dig.to_digit(10).unwrap();
        sum += x;
    }
    let x = start.elapsed();
    println!("answer 20: {sum}. T: {x:?}");
}
// d(n) is the sum of all proper divisors of n.
// (proper divisors: divors smaller than n)
// if d(a) == b and d(b) == a, where a != b,
// a and b are a amicable pair.
//Evaluate the sum of all the 
//amicable numbers under 10_000
// 31626
pub fn _21() {
    let start = Instant::now();

    let mut sum = 0;
    for a in 2..10_000 {
        let b = math::sum_divisors(a);
        if b > a {
            if math::sum_divisors(b) == a {
                sum += a + b;
            }
        } 
    }
    let x = start.elapsed();
    println!("answer 21: {sum}. T: {x:?}");
}
// sort the names in alphabetical order,
// get the alphabetical value of each name,
// multiply this by the name's position in the 
// list. What is the total of all the 
// name scores in the file?
// 871198282
pub fn _22() {
    let start = Instant::now();

    let mut file = include_str!("names.txt")
        .split("\",\"")
        .map(|x| x.as_bytes().to_owned())
        .collect::<Vec<_>>();

    let len = file.len();

    // remove quotes at start of 1st name.
    file[0].remove(0);
    //remove empty line at end of file.
    file[len-1].pop().unwrap();
    // remove quotes at end of last name.
    file[len-1].pop().unwrap();
    
    file.sort_unstable();

    let mut sum = 0;
    for i in 0..len {
        let mut temp: u32;

        temp = file[i].iter()
            .map(|ch| (ch - b'A'+1) as u32)
            .sum();

        // +1 is because names are 1-indexed.
        temp *= i as u32 + 1;
        sum += temp;
    }
    let x = start.elapsed();
    println!("answer 22: {sum}. T: {x:?}");
}
// abundant number(AN): sum of divisors is 
// greater than the number itself. 
// Find the sum of all integers <= 28123 
// which can't be written as the sum of two ANs.
// 4179871
pub fn _23() {
    let start = Instant::now();

    const LIMIT: usize =  28123; 

    // list of abundant numbers.
    let mut abun = Vec::with_capacity(7000);

    // find all abundant numbers until LIMIT.
    for n in 2..=LIMIT {
        let n = n as i32;
        if math::sum_divisors(n) > n {
            abun.push(n as usize);
        }
    }
    // sum of abundants.
    let mut soa = [false; LIMIT * 2 + 1];

    for i in 0..abun.len() {
        for j in 0..=i {
            soa[abun[i] + abun[j]] = true;
        }
    }
    let mut sum = 0;

    for i in 1..30_000 {
        if !soa[i] {
            sum += i;
        }
    }
    let x = start.elapsed();
    println!("answe 23: {sum}. T: {x:?}");
}

//  What is the millionth lexicographic 
//  permutation of the digits 0 to 9?
//  2783915460
pub fn _24() {
    type Us = usize;
    let start = Instant::now();

    let mut digits = [0, 1, 2, 
    3, 4, 5, 6, 7, 8, 9];

    const LEN: Us = 10;

    for _ in 1..=1_000_000 {

        let mut idx_k = 42;
        for k in 0..LEN - 1 {
            if digits[k] < digits[k+1] {
                idx_k = k;
            }
        }


        let mut idx_l = 42;
        for l in idx_k..LEN {
            if digits[idx_k] < digits[l] {
                idx_l = l;
            } 
        }
        digits.swap(idx_k, idx_l);
        
        let sub_arr = &mut digits[idx_k+1..LEN];
        sub_arr.reverse();
    }
    let x = start.elapsed();
    println!("answer 24: ");
    for e in digits {
        print!("{e}");
    }
    println!(" T: {x:?}");
}
// What is the index of the first 
// term in the Fibonacci sequence 
// to contain 1000 digits?
// 4782
#[allow(unused)]
pub fn _25() {
    let start = Instant::now();
    let mut index = 2;
    
    let mut x: BigDecimal;
    let mut y = BigDecimal::from(1);
    let mut z = y.clone();

    while z.digits() != 1000 {
        x = y.clone();
        y = z.clone();
        z = &x + &y;
        index += 1;
    }
    let x = start.elapsed();
    println!("answer 25: {index}. T: {x:?}");
}
