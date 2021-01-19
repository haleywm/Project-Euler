use euler::is_palindrome;

fn main() {
    // Generating all binary palindromes and then seeing if they're also regular palindromes
    // Even numbers, when made a palindrome, will start with 0 in binary so only doing odd ones
    const MAX: u64 = 1_000_000;

    let mut total = 0;

    for test in 1..=MAX {
        if is_palindrome(test, 10) && is_palindrome(test, 2) {
            total += test;
        }
    }

    /*
    let mut num = 1;
    'full_loop: loop {
        let len = get_len_base(num, 2);
        let mut mid_zeroes = 0;
        'zero_loop: loop {
            let mut new_num = num;
            for offset in 0..=len {
                new_num |= ((num >> offset) & 1) << (len + offset + mid_zeroes);
            }
            if new_num >= MAX {
                if mid_zeroes == 0 {
                    // Reached max value, halt whole program
                    break 'full_loop;
                }
                else {
                    // Just break this loop
                    break 'zero_loop;
                }
            }
            // Testing number
            if is_palindrome(new_num, 10) {
                //println!("{}", new_num);
                total += new_num;
            }

            mid_zeroes += 1;
        }
        
        num += 2;
    }
    */

    println!("{}", total);
}