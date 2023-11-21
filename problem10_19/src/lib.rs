use library::*;
use std::time::Instant;

pub mod lib1;
pub use lib1::*;
// Find the sum of all primes below 2 million.
// 142913828922
pub fn _10() {
    let start = Instant::now();
    const N: usize = 1_999_999;

    // implemented sieve of Eratosthenes.
    // because the naive algorithm was too slow.
    let mut is_prime = [true; N + 1];
    let mut p = 2;
    let mut i: usize;

    while p * p <= N {
        if is_prime[p] {

            i = p*p; 
            while i <= N {
                is_prime[i] = false;
                i += p;
            } 
        }
        p += 1;
    }

    let mut sum = 0;
    
    for i in 2..is_prime.len() {
        if is_prime[i] {
            sum += i;
        }
    }

    let x = start.elapsed();
    println!("answer 10: {sum}. T: {x:?}");
}
// What is the greatest product of four adjacent 
// numbers in the same direction 
// (up, down, left, right, or diagonally) in the grid?
// 70600674
pub fn _11() {
    let start = Instant::now();
    let grid: Vec<_> = include_str!("grid.txt")
        .lines().collect();

    let grid: Vec<_> = grid.iter()
        .map(|x| x.split(" ")
             .map(|y| y.parse().unwrap())
             .collect::<Vec<usize>>())
        .collect();

    let mut max = 0;
    for i in 0..20 {
        for j in 0..20 {

            let (mut right, mut left) = (1, 1);
            let (mut up, mut down) = (1, 1);
            let (mut dia1, mut dia2) = (1, 1);
            let (mut dia3, mut dia4) = (1, 1);

            let j16 = j <= 16;
            let j3 = j >= 3;
            let i3 = i >= 3;
            let _i16 = i <= 16;

            for k in 0..=3 {

                if j16 {
                    right *= grid[i][j + k];
                }
                if j3 {
                    left *= grid[i][j - k];
                }
                if i3 {
                    up *= grid[i - k][j];
                }
                 if _i16 {
                    down *= grid[i + k][j];
                }
                // dia is short for diagonal;

                // up to down, left to right
                if _i16 && j16 {
                    dia1 *= grid[i + k][j + k];
                }
                // up to down, right to left
                if _i16 && j3 {
                    dia2 *= grid[i + k][j - k];
                } 
                // down to up, left to right
                if i3 && j16 {
                    dia3 *= grid[i - k][j + k];
                }
                // down to up, right to left
                if i3 && j3 {
                    dia4 *= grid[i - k][j - k];
                }
            } 
            
            let x = [up, down, left, right, 
            dia1, dia2, dia3, dia4]
                .into_iter().max().unwrap();

            if x > max {
                max = x;
            }
        }
    }
    println!("answer 11: {max}. T: {:?}", start.elapsed());
}
// what is the value of the 1st triangle 
// number to have over 500 divisors.
// 76576500
pub fn _12() {
    let start = Instant::now();

    let mut num: usize;
    let mut i = 1usize;
    let x = loop {
        num = 0;
        for j in 1..=i {
            num += j;
        }
        let mut idx = 2;
        let initial_num = num;
        let mut num_factors = 1;

        while idx * idx <= initial_num {
            let mut power = 0;

            while num % idx == 0 {
                num /= idx;
                power += 1;
            }
            num_factors *= power + 1;
            idx += 1;
        }

        if num_factors > 500 {
            break initial_num;
        }
        i += 1;
    };
    println!("answer 12: {x}. T: {:?}", start.elapsed());
}
// find the 1st 10 digits of the sum 
// of the 100 50-digit numbers.
// 5537376230
pub fn _13() {
    let start = Instant::now();
    let list = include_str!("large_sum.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut nums = vec![];
    for i in 0..list.len() {
        nums.push(big_int::new(list[i]));
    }

    let mut sum = big_int::new("0");

    for i in 0..nums.len() {
        big_int::add_assign(&mut sum, &nums[i]);
    }
    print!("answer 13: ");
    big_int::print(&sum, 10); 
    println!(" T: {:?}", start.elapsed());
}
// in the collatz conjecture
// Which starting number, under one million, 
// produces the longest chain?
// 837799
pub fn _14() {
    let start = Instant::now();

    let mut num = 0;
    let mut count = 0;
    let mut max = 0;
    // HI: highest number.
    const HI: usize = 1_000_000;
    let mut cache = [0; HI];
    for i in 2..HI {

        let mut x = i;
        while x != 1 {
            if x < HI && cache[x] != 0 {
                count += cache[x];
                break;
            }
            // check if is even.
            if x & 1 == 0 {
                x /= 2;
                count += 1;
            } else {
                x *= 3;
                x += 1;
                // when n is odd, 3n + 1 is always even.
                x /= 2;
                count += 2;
            }
        }
        // store amount of steps in cache.
        cache[i] = count;
        if count > max {
            max = count;
            num = i;
        }
        count = 0;
    }
    let x = start.elapsed();
    println!("answer 14: {num}. T: {x:?}");
}
