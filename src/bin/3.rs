use primal::Primes;

fn main() {
    const INPUT: usize = 600851475143;

    let mut cur = INPUT;
    let mut last = 0;
    for prime in Primes::all() {
        while cur % prime == 0 {
            cur /= prime;
            last = prime;
        }
        if prime > cur {
            println!("{} ({} remaining)", last, cur);
            break;
        }
    }
}