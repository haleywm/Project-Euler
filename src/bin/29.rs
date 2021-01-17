fn main() {
    // First solution that comes to mind is to use a hashset of bigints, I want to do better though
    // Trying a vector of floats, compared within a tolerance
    const MAX: usize = 100;
    let mut numbers: Vec<f64> = Vec::with_capacity((MAX - 1).pow(2));

    for a in 2..=MAX {
        let a = a as f64;
        'num_check: for b in 2..=MAX {
            let b = b as f64;

            let num = b * a.ln();
            for &comp in numbers.iter() {
                if equal_enough(num, comp) {
                    // Already contains value
                    continue 'num_check;
                }
            }
            // Not in set
            numbers.push(num);
        }
    }

    println!("{}", numbers.len());
}

fn equal_enough(a: f64, b: f64) -> bool {
    // 8 zeroes
    const TOL: f64 = 0.000000001;

    (a - b).abs() < TOL
}