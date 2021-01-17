use num::rational::Ratio;

fn main() {
    let mut total = Ratio::new(1, 1);
    for digit_one in 1..=9 {
        for digit_two in 1..=9 {
            // Got digits of first number
            let digits = [digit_one, digit_two];
            for to_swap in 0..=1 {
                for alt_digit in 1..=9 {
                    let mut comp_digits = [0; 2];
                    comp_digits[to_swap] = alt_digit;
                    comp_digits[1 - to_swap] = digits[to_swap];
                    let num = to_num(digits);
                    let comp = to_num(comp_digits);
                    let fract = Ratio::new(num, comp);
                    if num < comp && (fract == Ratio::new(digits[1 - to_swap], comp_digits[to_swap])) {
                        total *= fract;
                    }
                }
            }
        }
    }
    println!("{}", total.denom());
    println!("{}", total.denom());
}

fn to_num(vals: [u32; 2]) -> u32 {
    vals[0] * 10 + vals[1]
}