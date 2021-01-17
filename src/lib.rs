pub fn div_sum(val: u64) -> u64 {
    // Getting all divisors of val, and summing them
    // Start at 1 as 1 is divisor
    let mut total = 1;
    for div in 2..=((val as f64).sqrt() as u64) {
        if val % div == 0 {
            // Found matching pairs
            total += div;
            let alt = val / div;
            if alt != div {
                total += alt;
            }
        }
    }

    total
}

pub fn factorial(val: u64) -> u64 {
    // Ensuring no overflow
    assert!(val <= 20);
    let mut result = 1;
    for i in 2..=val {
        result *= i;
    }
    result
}

pub fn is_palindrome(val: u64) -> bool {
    let digits = get_len_base(val, 10) as u32;
    for i in 0..digits/2 {
        if (val / 10u64.pow(i)) % 10 != (val / 10u64.pow(digits - i - 1)) % 10 {
            // No good
            return false;
        }
    }

    // Is palindrome!
    return true;
}

pub fn get_len_base(mut num: u64, base: u64) -> u64 {
    let mut len = 0;
    while num > 0 {
        len += 1;
        num /= base;
    }
    len
}