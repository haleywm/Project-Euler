use euler::factorial;

fn main() {
    // 9! * 7 = 2540160, 9! * 8 is still only 7 digits long so I'll just cap out at that :)
    let max = 2540160;

    let total: u64 = (10..max)
        .filter(move |&x| {
            let mut reduce = x;
            let mut total = 0;
            while reduce > 0 {
                total += factorial(reduce % 10);
                reduce /= 10;
            }
            total == x
        })
        .sum();
    
    println!("{}", total);
}