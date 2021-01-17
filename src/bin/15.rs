fn main() {
    const GRID_SIZE: u64 = 20;

    let mut total: u64 = 1;

    for i in 1..=GRID_SIZE {
        total *= 2 * GRID_SIZE + 1 - i;
        total /= i;
    }

    println!("{}", total);
}