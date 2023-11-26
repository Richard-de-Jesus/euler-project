use std::time::Instant;
use bigdecimal::num_bigint::BigUint;
// in the collatz conjecture
// Which starting number, under one million, 
// produces the longest chain?
// 837799
pub fn _14() {
    let start = Instant::now();

    let mut num = 0; 
    let mut count = 0; 
    let mut max = 0; 
    //HI: highest number.
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
                x >>= 1;
                count += 1;
            } else {
                x *= 3;
                x += 1;
                // when n is odd, 3n + 1 is always even.
                x >>= 2;
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
    println!("jsiajsoiasji");
    println!("answer 14: {num}. T: {x:?}");
}
// how many lattice path are there 
// in a 20x20 grid?
// 137846528820
pub fn _15() {
    let start = Instant::now();
    let k = 20u128;
    
    let mut num = 1;
    //binomial coefficient numbers: 20 and 40(n)
    // n! / k! * (n-k)! | we reduce 40! 
    // to 40*39*38...*21 and remove (n-k)! 
    for i in k+1..=40 {
        num *= i;
    } 
    // divide num by k!
    for i in 1..=k {
        num /= i;
    }
    let x = start.elapsed();
    println!("answer 15: {num}. T: {x:?}");
}
// What is the sum of the digits of 2^1000?
// 1366
pub fn _16() {
    let start = Instant::now();
    let mut num =  BigUint::from(2u32);

    for _ in 2..=1000 {
        num *= 2u32;
    }
    let mut sum = 0;
    let digits = num.to_str_radix(10);

    for ch in digits.chars() {
        let x = ch.to_digit(10).unwrap();
        sum += x;
    }
    let x = start.elapsed();
    println!("answer 16: {sum}. T: {x:?}");
}
//  if all the numbers from 1 to (one thousand) inclusive 
//  were written out in words, how many letters would be used?
//  note: do not count spaces/hyphens, 'and' is counted.
//  21124
pub fn _17() {
    let start = Instant::now();
    // 1st element is empty, so index allign with number.
    let units = ["", "one", "two", "three", "four", 
    "five", "six", "seven", "eight", "nine"];

    let special_dozens = ["ten", "eleven", "twelve", 
    "thirteen", "fourteen", "fifteen", "sixteen", 
    "seventeen", "eighteen", "nineteen"];

    // same here, 20 idx = 2, 30 idx = 3 and so on;
    let dozens = ["", "", "twenty", "thirty", "forty", 
    "fifty", "sixty", "seventy", "eighty", "ninety"];

    let sum = &mut 0usize as *mut usize;
    let add = |x: &str| unsafe {*sum += x.len()};

    for n in 1..=1000 {
        // format! left pads n to 4 digits with '0';
        let digits: Vec<_> = format!("{n:0>4}")
            .chars().map(|x| x.to_digit(10).unwrap())
            .collect();

        // assign position of each digit.
        let thousand = digits[0] as usize;
        let hundred = digits[1] as usize;
        let dozen = digits[2] as usize;
        let unit = digits[3] as usize;

        // if dozen digit is 1 then
        // check the special_dozens array element 
        // on the index of the unit digit.
        if dozen == 1 {
            add(special_dozens[unit]);
        } else {
            add(units[unit]);
        }
        add(dozens[dozen]);

        if hundred > 0 {
            add(units[hundred]);
            add("hundred");
            if unit > 0 || dozen > 0  {
               add("and");
            }
        } 
        if thousand > 0 {
            add(units[thousand]);
            add("thousand");
        }
    }
    unsafe { let x = start.elapsed();
    println!("answer 17: {}. T: {x:?}", *sum);}
}
// Find the maximum total from top to bottom 
// of the triangle below:
// 1074
pub fn _18() {
    let start = Instant::now();
    // we can only add adjacent elements(AE).
    // AEs are the 2 numbers in the next rom,
    // the 1st have same index, 2nd have index + 1.
    let mut tri = include_str!("triangle.txt")
        .split("\n")
        .map(|ln| ln.split(" ").collect::<Vec<&str>>())
        .map(|xs| xs.iter()
             .map(|x| x.parse()
                  .unwrap_or(0)).collect::<Vec<u16>>())
        .collect::<Vec<Vec<u16>>>();
    // removing the empty line in the end.
    tri.pop().unwrap();

    while tri.len() > 1 {
        let last  = tri.pop().unwrap();
        let ante = tri.pop().unwrap();

        let mut new = Vec::new();

        for (i, val) in ante.iter().enumerate() {
            new.push(last[i].max(last[i+1]) + val);
        }
        tri.push(new);
    }
    let x = start.elapsed();
    println!("answer 18: {:?}. T: {x:?}", tri[0][0]);
}
// How many Sundays fell on the first of the month
// during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
// 171
pub fn _19() { 
    let start = Instant::now();
    // number of sundays in 1st day of the month
    let mut sundays = 0;
    // number of days in each month.
    let mut months_len = [31, 28, 31, 30, 
    31, 30, 31, 31, 30, 31, 30, 31];
    // week days 0-indexed. 2 is tuesday.
    let mut week_day = 2;
    
    for year in 1901..=2000 {

        if year & 3 == 0 {
            months_len[1] = 29;
        } else {
            months_len[1] = 28;
        }

        for month in 0..12 {
            let days = months_len[month];

            week_day += days % 7;
            if week_day % 7 == 0 {
                sundays += 1;
            } 
        }
    }
    let x = start.elapsed();
    println!("answer 19: {sundays}. T: {x:?}");
}

