fn main() {
    const MAX: u32 = 1000;
    
    let d = (2..MAX).max_by_key(|x| get_div_len(*x)).unwrap();

    println!("{}", d);
}

fn get_div_len(val: u32) -> u32 {
    if val % 2 == 0 || val % 5 == 0 {
        return 0;
    }
    // Filtered out values that would cause loops
    let mut rem = 10 % val;
    let mut count = 0;
    while rem != 1 {
        rem = (rem * 10) % val;
        count += 1;
    }
    count
}