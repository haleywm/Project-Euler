fn main() {
    // Input: The number of divisors a number must have
    const DIVCOUNT: u64 = 500;

    let mut cur: u64 = 0;
    let mut i = 1;

    loop {
        cur += i;
        i += 1;
        // Getting divisors of cur
        // Interating for all values from 1 to sqrt of cur and testing
        // Each divisor must have a counterpart, unless cur is a square number
        let mut count = 0;
        let limit = (cur as f64).sqrt() as u64;
        for test in 1..=limit {
            if cur % test == 0 {
                count += 2;
            }
        }
        
        // Then testing sqrt itself, as it was likely rounded down
        // If it wasn't then it won't have a counterpart, so sub 1
        if limit.pow(2) == cur {
            count -= 1;
        }
        if count > DIVCOUNT {
            println!("{}", cur);
            break;
        }
    }
}