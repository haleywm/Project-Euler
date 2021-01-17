use primal::Sieve;
use std::convert::TryInto;

fn main() {
    // First, generating a set of primes for use
    const MAX: usize = 1000;
    // Guessing largest possible quadratic formula within this is 200
    let max_n_guess: usize = 200;

    let largest_possible =  max_n_guess.pow(2) + (MAX - 1) * max_n_guess + MAX;
    let primes = Sieve::new(largest_possible);

    // Then, guessing different possible values of a and b to find the maximum chain length
    let max = MAX as i64;
    let min = max * -1;

    let values = (min + 1..max).flat_map(|x| {
        // n = 0 must be prime, so just going through primes less than or equal to MAX
        primes.primes_from(2).take_while(|&x| x <= MAX).map(move |y| (x, y as i64) )
    });

    let (a, b) = values
        .max_by_key(|&(a, b)| {
            let mut n: i64 = 0;
            // If val is negative turn into 0, which isn't prime
            while primes.is_prime((n.pow(2) + a * n + b).try_into().unwrap_or_default()) {
                n += 1;
            }
            n
        })
        .unwrap();
    
    println!("{} * {} = {}", a, b, a * b);

    let mut n: i64 = 0;
    // If val is negative turn into 0, which isn't prime
    while primes.is_prime((n.pow(2) + a * n + b).try_into().unwrap_or_default()) {
        n += 1;
    }
    println!("{}", n);
}