use primal::Sieve;

fn main() {
    // Generating numbers that are left to right truncatable and seeing if they are also right to left truncatable
    const LIMIT: usize = 11;
    const MAX: usize = 1_000_000;
    const VALID_DIGITS: [usize; 6] = [1, 2, 3, 5, 7, 9];

    let primes = Sieve::new(MAX);

    // 2 and 5 only work for single digit candidates so just putting them here
    let mut candidates: Vec<usize> = vec![3, 7];
    let mut total = 0;
    let mut found = 0;
    while found < LIMIT {
        let next = candidates.pop().unwrap();
        // Checking if it's right truncatable
        if primes.is_prime(next) {
            let mut test = next / 10;
            let mut matches = true;
            let mut mult = 10;
            while test > 0 {
                if !primes.is_prime(test) {
                    matches = false;
                }
                test /= 10;
                mult *= 10;
            }
            if matches && next > 10 {
                // Found one!
                println!("{}", next);
                total += next;
                found += 1;
            }
            // Generating next candidates
            for &add in VALID_DIGITS.iter() {
                let to_add = add * mult + next;
                if to_add < MAX {
                    candidates.push(to_add);
                }
            }
        }
        
    }

    println!("{}", total);
}