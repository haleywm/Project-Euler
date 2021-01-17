use std::collections::HashSet;

use euler::div_sum;

fn main() {
    const MAX: u64 = 10000;
    let mut pairs = HashSet::new();

    for num in 2..MAX {
        if pairs.contains(&num) {
            continue;
        }
        let val = div_sum(num);
        let possible_pair = div_sum(val);
        if possible_pair == num && val != num {
            // Found a match!
            pairs.insert(num);
            pairs.insert(val);
        }
    }

    println!("{:?}", pairs);

    let total: u64 = pairs.into_iter().filter(|&x| x < MAX).sum();

    println!("{}", total);
}
