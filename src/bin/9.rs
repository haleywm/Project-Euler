fn main() {
    const TOTAL: usize = 1000;

    for a in 1..=(TOTAL - 2) {
        let rem = TOTAL - a;
        // If rem is even, make c = b + 2, otherwise c = b + 1
        let (mut b, mut c) = if rem % 2 == 0 {
            (rem / 2 - 1, rem / 2 + 1)
        }
        else {
            (rem / 2, TOTAL - (rem / 2))
        };

        while b > a && a.pow(2) + b.pow(2) > c.pow(2) {
            // Decrease b and increase c
            b -= 1;
            c += 1;
        }
        if a.pow(2) + b.pow(2) == c.pow(2) {
            println!("{}", a * b * c);
            break;
        }
    }
}