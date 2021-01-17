fn main() {
    const INPUT: usize = 10001;
    let mut i = 1;
    let mut cur = 1;

    while i < INPUT {
        cur += 2;
        let mut prime = true;
        let root = (cur as f32).sqrt() as u32;
        for comp in (3..=root).step_by(2) {
            if cur % comp == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            i += 1;
        }
    }

    println!("{}", cur);
}