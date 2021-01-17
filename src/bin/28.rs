fn main() {
    const SIZE: usize = 1001;

    let max = SIZE * SIZE;
    
    let mut total = 0;
    let mut cur = 1;
    let mut inc = 2;
    let mut times = 0;

    while cur <= max {
        total += cur;
        cur += inc;
        times += 1;
        if times == 4 {
            // Met all 4 corners, time to increase the increment
            inc += 2;
            times = 0;
        }
    }

    println!("{}", total);
}