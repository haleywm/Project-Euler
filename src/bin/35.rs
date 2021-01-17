use primal::Sieve;

fn main() {
    const MAX: usize = 1_000_000;

    let sieve = Sieve::new(MAX * 10);

    let mut circular = 0;
    'prime_check: for prime in sieve.primes_from(2).take_while(|&x| x < MAX) {
        // Getting all variations
        let len = (prime as f64).log10() as u32;

        for rot in 0..=len {
            let adjust = 10usize.pow(rot);
            let new_num = (prime % adjust) * 10usize.pow(len - rot + 1) + (prime / adjust);

            if !sieve.is_prime(new_num) {
                continue 'prime_check;
            }
        }
        // Number must be prime
        circular += 1;
    }
    println!("{}", circular);
}