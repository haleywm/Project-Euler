use euler::get_len_base;

fn main() {
    const MAX_N: u64 = 10_000;

    let result = (1..MAX_N)
        .filter_map(|n| {
            // Converting into 9 digit number
            let mut total = 0;
            let mut mult = 1;

            while total < 100_000_000 {
                let pow = get_len_base(n * mult, 10) as u32;
                total = total * 10u64.pow(pow) + n * mult;
                mult += 1;
            }

            // Determining if pandigital
            let mut to_div = total;
            let mut digits = [true; 10];
            digits[0] = false;
            while to_div > 0 {
                let digit = (to_div % 10) as usize;
                to_div /= 10;
                if !digits[digit] {
                    // Uh oh
                    return None;
                }
                else {
                    digits[digit] = false;
                }
            }
            // Of it made it through this yippee
            Some(total)
        })
        .max()
        .unwrap();
    
    println!("{}", result);
}