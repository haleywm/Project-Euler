fn main() {
    const POWER: u32 = 5;

    // Calculating limit
    let mut digits = 1;
    let max_value = loop {
        let max = digits * 9u64.pow(POWER);
        let mut max_digits = 0;
        let mut max_reduce = max;
        while max_reduce > 0 {
            max_reduce /= 10;
            max_digits += 1;
        }
        if max_digits <= digits {
            // Found a maximum
            break max;
        }
        digits += 1;
    };

    let total: u64 = (2..=max_value)
        .filter_map(|number| {
            // Getting the total of each digit to the power of POWER
            let mut total = 0;
            let mut to_reduce = number;
            while to_reduce > 0 {
                total += (to_reduce % 10).pow(POWER);
                to_reduce /= 10;
            }
            // Then seeing if the value fits
            if number == total {
                Some(number)
            }
            else {
                None
            }
        })
        .sum();
    
    println!("{}", total);
}