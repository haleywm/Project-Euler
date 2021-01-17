use std::cmp::min;

fn main() {
    const MAX: u64 = 9;
    const GET: u64 = 1_000_000;
    let vals: Vec<u64> = (0..=MAX).collect();
    let lehmer = num_to_lehmer(GET - 1, MAX as usize + 1);

    let result = decode_lehmer(vals, lehmer);
    for num in result {
        print!("{}", num);
    }
    println!();
}

fn num_to_lehmer(num: u64, len: usize) -> Vec<usize> {
    // Converts a given number to a lehmer code
    // Start radix at 2, as 1 will always have a value of 0 so leave it out
    let mut num = num as usize;
    let mut radix = 2;
    let mut result = Vec::with_capacity(len);
    result.push(0);
    while num > 0 {
        // Getting remained and adding
        result.push(num % radix);
        num /= radix;
        radix += 1;
    }
    for _ in result.len()..len {
        result.push(0);
    }

    result
}

fn decode_lehmer(mut range: Vec<u64>, lehmer: Vec<usize>) -> Vec<u64> {
    assert_eq!(range.len(), lehmer.len());
    let mut output = Vec::with_capacity(range.len());
    for pos in lehmer.into_iter().rev() {
        output.push(range.remove(min(pos, range.len() - 1)));
    }
    output
}