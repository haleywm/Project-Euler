use primal::Sieve;

fn main() {
    // Trying a max value of 1 million as calculating an amount to start with makes checking for primeness easier
    const MAX: usize = 1000_000;
    
    let primes = Sieve::new(MAX);
    let mut successful = false;

    'primary: for prime in primes.primes_from(100) {
        // Making a vec of digits
        let mut digits = Vec::new();
        let mut to_reduce = prime;
        while to_reduce != 0 {
            digits.push(to_reduce % 10);
            to_reduce /= 10;
        }
        // Then iterating through each combination of values to replace using bitwise indication
        for to_rep in 1..2usize.pow(digits.len() as u32) - 1 {
            // Then keeping count of if each is prime
            let mut total = 0;
            for x in 0..=9 {
                let num = rep_vals(&digits, x, to_rep);
                if primes.is_prime(num) {
                    total += 1;
                }
            }
            // Then lastly printing the number if the number has an appropriate number of primes
            if total == 8 {
                println!("Found val: {}", rep_vals(&digits, 1, to_rep));
                successful = true;
                break 'primary;
            }
        }
    }
    if !successful {
        println!("Couldn't find a matching prime, maybe try increasing max");
    }
}

fn rep_vals(digits: &Vec<usize>, digit: usize, to_rep: usize) -> usize {
    // Only do x=0 if the first digit isn't being replaces
    if digit != 0 || (to_rep >> (digits.len() - 1) == 0) {
        let mut num = 0;
        for i in 0..digits.len() {
            num += 10usize.pow(i as u32) *
                if (to_rep >> i) & 1 == 0 {
                    // Reg val
                    digits[i]
                }
                else {
                    digit
                };
        }
        return num;
    }
    // Return 0 otherwise
    return 0;
}