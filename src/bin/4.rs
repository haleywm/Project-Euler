use euler::is_palindrome;

fn main() {
    const MIN: u64 = 111;
    const MAX: u64 = 999;

    let result = (MIN..=MAX).flat_map(|x| (x..=MAX).map(move |y| x * y))
        .filter(|&num| is_palindrome(num, 10))
        .max()
        .unwrap();
    
    println!("{}", result);
}
