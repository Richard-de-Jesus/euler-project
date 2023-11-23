use bigdecimal::BigDecimal;

type Us = usize;
// find the value of d < 1000 for 
// wich 1 / d contains the largest 
// recurring cycle in the decimal part.
// 983
#[allow(unused)]
pub fn _26() {
    use BigDecimal as BD;
    // the 1 / d part.
    let mut d = 2;
    // greatest cycle size.
    let mut max_cyle = 2;

    const PRECISION: usize = 100;

    let is_cycle = |num: &[u8], k: Us| {

        dbg!(&num[0..11]);
        let section = num.chunks_exact(k);
        dbg!(section.len());
        

        true
    }; 

    'outer: for i in 4..10 {

        let unit = (BD::from(1) / i as u32)
            .to_string();

        let decimals = unit[2..].as_bytes();
        if decimals.len() < PRECISION {
            continue;
        }

        
        for k in max_cyle..= PRECISION / 2 {


            is_cycle(decimals, k);
            break 'outer;

            if is_cycle(decimals, k) {
                max_cyle = k;
                d = i;
            }
            
        }
        

    } 
}
