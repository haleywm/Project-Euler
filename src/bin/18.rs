use std::io::{stdin, BufRead};
use std::cmp::max;

fn main() {
    // First, read a traingle in from stdin
    let mut triangle: Vec<Vec<u64>> = Vec::new();

    for line in stdin().lock().lines() {
        // If line isn't valid or is empty break
        match line {
            Ok(line) => {
                if line.len() == 0 {
                    break;
                }
                let numbers: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
                // Ensuring triangle shape by ensuring first line has length 1, and following vals are 1 more than prev
                let last_len = triangle.last().map_or(1, |x| x.len() + 1);
                assert_eq!(last_len, numbers.len());
                triangle.push(numbers);
            }
            Err(_) => break,
        }
    }
    // Done parsing
    assert!(triangle.len() > 0);
    // I will use a 'trickle up' method, where starting from the second lowest row and going upwards, I'll add the largest of the 2 below children
    for row in (0..triangle.len() - 1).rev() {
        for i in 0..triangle[row].len() {
            triangle[row][i] += max(triangle[row + 1][i], triangle[row + 1][i + 1]);
        }
    }

    println!("{}", triangle[0][0]);
}