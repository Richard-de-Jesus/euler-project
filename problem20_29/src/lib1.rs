use bigdecimal::BigDecimal;

type Us = usize;
// returns true if sub-string 
// of length k starting at index i 
// is also a prefix of the string 
fn is_prefix(num: &str, len: Us, mut i: Us, k: Us) -> bool {

    if i + k > len {
        return false;
    }

    let num = num.as_bytes();
    for j in 0..k {

        if num[i] != num[j] {
            return false;
        }
        i += 1;
    }
    false
}

// find the value of d < 1000 for 
// wich 1 / d contains the largest 
// recurring cycle in the decimal part.
#[allow(unused)]
pub fn _26() {
    use BigDecimal as BD;

    let mut max = 2;
    const PRECISION: usize = 100;

    let is_cycle = |num: &str, len: Us, k: Us| {
        let mut idx = k;
        while idx < len {
            if !is_prefix(num, len, idx, k) {
                return false;
            }
        }
        true
    }; 

    for i in 2..10 {

        let unit = (BD::from(1) / i as u32)
            .to_string();

        let decimals = &unit[2..];
        if decimals.len() < PRECISION {
            continue;
        }
        
        for k in max..= PRECISION / 2 {

            let len = PRECISION - (PRECISION % k);
            if is_cycle(decimals, len, k) {
                dbg!(decimals);
                dbg!(i);
                max = i;
            }
            
        }
        

    } 
}
