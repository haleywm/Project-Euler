fn main() {
    const INPUT: usize = 20;

    let mut val = 1;
    for next in 1..=INPUT {
        val = lcm(val, next);
    }

    println!("{}", val);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    }
    else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}