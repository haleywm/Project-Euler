use num::bigint::BigUint;

fn main() {
    const BASE: i64 = 1000;

    let to_use = BASE + 1;
    // Using base instead of to_use to simplify rounding with division to ensure we have enough
    let bytes_needed = ((BASE / 32) + 1) as usize;
    // Getting modulus of to_use over 8, then subbing one and bringing round to 7 if needed
    let last_offset = ((to_use % 32) - 1 + 32) % 32;

    let mut bytes = vec![0u32; bytes_needed];
    bytes[bytes_needed - 1] = 1 << last_offset;

    let num = BigUint::new(bytes);
    let result: u32 = num.to_str_radix(10).chars().map(|x| x.to_digit(10).unwrap()).sum();
    println!("{}", result);
}