use std::collections::HashSet;
use euler::div_sum;

fn main() {
    // Question states all numbers greater can be the sum of two abundant numbers
    const CAP: u64 = 28123;
    // Creating set of all abundant numbers
    let mut abundant = Vec::new();
    for num in 1..=CAP {
        let ds = div_sum(num);
        if ds > num {
            abundant.push(num);
        }
    }

    // Then iterating through the range again and finding numbers that can't be summed by two values
    let total: u64 = (1..=CAP)
        .filter(|&num| {
            let mut seen = HashSet::with_capacity(abundant.len());
            for &cmp in abundant.iter() {
                if cmp >= num {
                    break;
                }
                seen.insert(cmp);
                if seen.contains(&(num - cmp)) {
                    return false;
                }
            }
            // Unable to find match, so it can't be made with abundant numbers, add it
            return true;
        })
        .sum();
    
    println!("{}", total);
}