fn main() {
    const CURRENCIES: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

    let result = rec_ways(&CURRENCIES, 200);

    println!("{}", result);
}

fn rec_ways(units: &[u32], total: u32) -> u32 {
    // Recursively counts ways to add up
    // Base case: units has length of 1
    if units.len() == 1 {
        return 1;
    }
    else {
        let mut ways = 0;
        let last = units[units.len() - 1];
        let to_pass = &units[0..units.len() - 1];
        
        for mult in 0..=total/last {
            ways += rec_ways(to_pass, total - mult * last);
        }
        return ways;
    }
}