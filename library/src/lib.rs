use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::{Add, AddAssign, MulAssign, Mul, Shl, Shr};
use std::ops::{DivAssign, Div, Sub, SubAssign};


// bytes: all the digits,
// sign: true for positive, false for negative.
// negative numbers not supported now.
#[derive(Debug, Clone, Eq, PartialEq)]
struct BigInt {
    bytes: Vec<u8>,
    sign: bool,
}

impl BigInt {
    pub fn new(s: &str) -> Option<BigInt>  {
        
        if s.is_empty() {
            return None;
        }
        let s = s.as_bytes();

        let mut result = BigInt {
            bytes: Vec::with_capacity(s.len()),
            sign: true,
        };

        let mut iter = s.iter();
        // if there is a sign, iter.next will 
        // skip it, also adjust the sign.
        match s[0] {
            b'+' => { 
                iter.next();
            }
            b'-' => {
                todo!("negative BigInt");
                //result.sign = false;
                //iter.next();
            }
            _ => (),
        };
        // check each char.
        let error = iter
            .any(|&x| !(x.is_ascii_digit() || x == b'_'));

        if error {
            return None;
        }
        // remove sign and underscores.
        let iter = s.iter().rev()
            .filter(|x| x.is_ascii_digit());
        for e in iter {
            result.bytes.push(e - b'0');
        }
        Some(result)
    }
    // get BigInt from integers of any type.
    pub fn from_int<T: ToString + Sum>(int: T) -> Self {
        BigInt::new(&int.to_string()).unwrap()
    }
    
    pub fn to_int(&self) -> i128 {
        let mut sum = self.bytes[0] as i128;

        for i in 1..self.bytes.len() {
            let n = self.bytes[i] as usize * i * 10; 
            sum += n as i128;
        }
        sum
    }
    // get usize from BigInt.
    
    // get digits in internal order(reversed).
    pub fn raw_digits(&self) -> &Vec<u8> {
        &self.bytes
    }
    // get BigInt's digit in normal order.
    pub fn digits(&self) -> Vec<u8> {

        let mut temp = self.bytes.clone();
        temp.reverse();
        temp
    } 
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering as O;
        let a = &self.bytes;
        let b = &other.bytes;

        let s1 = self.sign;
        let s2 = other.sign;
        // if s1 == - and s2 == +.
        if (a.len() < b.len()) || (!s1 && s2)    {
            return O::Less;
        }
        // reverse here.
        if (a.len() > b.len()) || (s1 && !s2) {
            return O::Greater;
        }
        if self == other {
            return O::Equal;
        }

        let compare = |x: &Vec<u8>, y: &Vec<u8>| {
            for i in (0..x.len()).rev() {
                if x[i] < y[i] {
                    return O::Less;
                }
                if x[i] > y[i] {
                    return O::Greater;
                }
            }
            O::Equal
        };
        return match s1 {
            true => compare(a, b),
            false => compare(b, a),
        };
    }
}

impl Shl<&BigInt> for &BigInt {
    type Output = BigInt;
    // if rhs is larger than u32::MAX, it is 
    // reduced to u32::MAX.
    fn shl(self, rhs: &BigInt) -> Self::Output {

        let mut rhs = rhs.to_int() as u32;
        
        if rhs > u32::MAX {
            rhs = u32::MAX;
        }

        let right = 2u32.pow(rhs);
        self * &BigInt::from_int(right)
    }
}
impl Shl for BigInt {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        (&self).shl(&rhs)
    }
}
// not yet implemented
impl Shr<&BigInt> for &BigInt {
    type Output = BigInt;
    fn shr(self, rhs: &BigInt) -> Self::Output {

        let left = 2u32.pow(rhs.to_int() as u32);
        self * &BigInt::from_int(left)
    }
}
impl Shr for BigInt {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        (&self).shr(&rhs)
    }
}
impl AddAssign<&BigInt> for BigInt {

    fn add_assign(&mut self, rhs: &BigInt) {

        let mut carry = 0;
        let mut sum: u8;
        // decimal is base 10.
        const BASE: u8 = 10;

        let a = &mut self.bytes;
        let b = &rhs.bytes;

        let len = b.len();
        // if a have less digits than b.
        while a.len() < len {
            a.push(0);
        }

        if self.sign == rhs.sign {
            // iterate for lenght of smaller num.
            for i in 0..len {
                sum = a[i] + b[i] + carry;

                if sum >=BASE {
                    sum -= BASE;
                    carry = 1;
                } else {
                    carry = 0;
                }
                a[i] = sum;
            }

            if carry > 0 {
                a.push(1);
            }
        } else {
            todo!("addition with opposite signs");
        }
    } 
}
impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, rhs: BigInt) {
        self.add_assign(&rhs);
    }
}

impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> Self::Output {
        let mut temp = self.clone();
        temp += rhs;
        temp
    }
}
impl Add<BigInt> for BigInt {
    type Output = BigInt;
    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl SubAssign<&BigInt> for BigInt {
    // if a < b, it is not supported
    // and a is set to 0.
    #[allow(unused_assignments)]
    fn sub_assign(&mut self, rhs: &BigInt) {
        
        // decimal is base 10.
        const BASE: i8 = 10;
        // not supported for now.
        if *self < *rhs {
            *self = BigInt::from_int(0);
            return;
        }

        let a = &mut self.bytes;
        let b = &rhs.bytes;


        if self.sign == rhs.sign {

            let mut borrow = 0i8;
            for i in 0..a.len() {
                
                let ai8 = a[i] as i8;
                
                let mut diff = ai8 - borrow;
                // avoid out of bounds acess.
                if i < b.len() {
                    diff -= b[i] as i8;
                }

                if diff < 0 {
                    diff += BASE;
                    borrow = 1;
                } else {
                    borrow = 0;
                }
                a[i] = diff as u8;
            }
        } else {
            todo!("subtracion with opposite signs");
        }
        // remove leading zeroes.
        while a.last() == Some(&0) {
            a.pop().unwrap();
        }
    }
}
impl SubAssign<BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: BigInt) {
        self.sub_assign(&rhs);
    }
}

impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;

    fn sub(self, rhs: &BigInt) -> Self::Output {
        let mut temp = self.clone();
        temp -= rhs;
        temp
    }
}
impl Sub<BigInt> for BigInt {
    type Output = Self;

    fn sub(self, rhs: BigInt) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl MulAssign<&BigInt> for BigInt {

    fn mul_assign(&mut self, rhs: &BigInt) {

        if self.sign == rhs.sign {
            self.sign = true;
        } else {
            self.sign = false;
        }
        let a = &mut self.bytes;
        let b = &rhs.bytes;

        // multiplication by zero.
        let zero = vec![0u8];
        if *a == zero || *b == zero {
            *a = zero;
        }
        // multiplication by one;
        let one = vec![1u8];
        if *a == one {
            *a = b.clone();
        }
        if *b == one {
            return;
        }

        let (mut n, m) = (a.len(), b.len());
        let mut v = vec![0; m + n];

        for i in 0..n {
            for j in 0..m {
                v[i+j] += a[i] * b[j]; 
            }
        }
        n += m;
        a.resize(v.len(), 0);

        let mut t = 0;
        let mut s: u16;
        for i in 0..n {

            s = t as u16 + v[i] as u16;
            v[i] = (s % 10) as u8;

            t = (s / 10) as u8;
            a[i] = v[i];
        }
        
        let mut i = n - 1;
        while i >= 1 && v[i] == 0 {
            a.pop().unwrap();
            i -= 1;
        }
    }
}
impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        self.mul_assign(&rhs);
    }
}
impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &BigInt) -> Self::Output {

        let mut temp = self.clone();
        temp *= rhs;
        temp
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl DivAssign<&BigInt> for BigInt {
    #[allow(unused)]
    fn div_assign(&mut self, rhs: &BigInt) {
        
        let new = |x| {
            BigInt::new(x).unwrap()
        };
        // if rhs is 1;
        if rhs.bytes == vec![1u8] {
            return;
        }
        // division by 0 and a < b.
        if rhs.bytes == vec![0u8] || *self < *rhs {
            *self = new("0"); 
            return;
        }
        if *self == *rhs {
            *self = new("1");
            return;
        } 
        let a = &mut self.bytes;
        let b = &rhs.bytes;


    }
}

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

#[cfg(test)]
mod test {

    use crate::BigInt;

    
    #[test]
    fn big_int_misc() {
        use BigInt as B;
        let new = |x| B::new(x).unwrap(); 
        let fint = |x| B::from_int(x);

        let mut num = fint(8721_i64);
        // big_int digits are stored in reverse.
        assert_eq!(vec![1, 2, 7, 8], num.bytes);

        num = fint(345);
        assert_eq!(num.digits(), vec![3, 4, 5]);
        assert_eq!(num.raw_digits(), &num.bytes);
        
        // test signs.
        num = new("+123");
        assert_eq!(fint(123), num);

        // will skip negative for now.
        //num = new("-123");
        //assert_eq!(fint(-123), num);

        // test use of underscore.
        num = new("_400_000__000_");
        num += num.clone();
        assert_eq!(new("800_000_000"), num);

        // create BigInt with x, check if return None.
        let fails = |x| assert_eq!(None, B::new(x));
        // checking failure conditions.
        fails("1232+");
        fails("489-");
        fails("13a29");
    }
    #[test]
    fn big_int_sum_and_sub() {
        let fint = |x: i64| BigInt::from_int(x);

        let test_add = |x, y| {
            let a = fint(x) + fint(y);
            assert_eq!(fint(x + y), a);
        };

        // a.len == b.len;
        test_add(53_221, 99_999);
        // a.len > b.len;
        test_add(153_220, 5);
        // a.len < b.len;
        test_add(42, 1_000_1_264_336);

        let test_sub = |x, y| {
            let a = fint(x) - fint(y);
            assert_eq!(fint(x - y), a);
        };

        test_sub(6, 2);
        test_sub(19, 10);
        test_sub(12, 2);
        test_sub(1111, 22);
        test_sub(2023, 1066);
    } 
    #[test]
    fn big_int_shift(){

        let fint = |x: i64| BigInt::from_int(x);

        let sh_left = |x, y| {
            let a = fint(x) << fint(y);
            
            let result = x << y;
            assert_eq!(fint(result), a);
        };

        let _sh_right = |x, y| {
            let a = fint(x) >> fint(y);

            let result = x >> y;
            assert_eq!(fint(result), a);
        };

        sh_left(16, 2);
        sh_left(123, 2);
        sh_left(1, 10);
        sh_left(21892393, 2);
        sh_left(0, 1);

        /*sh_right(1500, 2);
        sh_right(123, 1);
        sh_right(210939233321, 5);
        sh_right(1000000000000000, 8);
        sh_right(0, 1);*/

    }
    #[test]
    fn big_int_mul_and_div() {
        
        let fint = |x| BigInt::from_int(x);
        // a.len == b.len.
        let mut num = fint(424_242);
        num *= fint(212_121);
        assert_eq!(fint(89_990_637_282_i64), num);
        // a.len > b.len.
        num = fint(2233);
        num *= fint(3);
        assert_eq!(fint(6699), num);

    }
    #[test]
    fn big_int_comp() {

        let fint = |x| BigInt::from_int(x);

        let num = fint(123);

        assert_eq!(num, fint(123));
        assert_ne!(num, fint(223));

        assert!(num > fint(122));
        assert!(num > fint(1));

        assert!(num < fint(124));
        assert!(num < fint(2891228));
    }
}



