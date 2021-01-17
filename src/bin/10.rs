fn main() {
    const INPUT: usize = 2_000_000;
    let mut cur = 3;
    let mut total = 2;

    while cur < INPUT {
        let mut prime = true;
        let root = (cur as f32).sqrt() as usize;
        for comp in (3..=root).step_by(2) {
            if cur % comp == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            total += cur;
        }
        cur += 2;
    }

    println!("{}", total);
}