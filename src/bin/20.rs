use num::bigint::BigUint;

fn main() {
    const IN: usize = 100;

    let num = (2..IN).fold(BigUint::from(IN), |total, mult| total * mult);
    let total: u64 = num.to_radix_be(10).iter().map(|x| *x as u64).sum();
    println!("{}", total);
}