use num::bigint::BigUint;
use num::traits::Zero;

struct Fib {
    cur: BigUint,
    acc: BigUint,
}

impl Fib {
    fn new() -> Fib {
        Fib { cur: BigUint::from(1u32), acc: BigUint::from(0u32) }
    }
}

impl Iterator for Fib {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        {
            let tmp = self.cur.clone();
            self.cur += &self.acc;
            self.acc = tmp;
        }
        
        Some(self.cur.clone())
    }
}

fn main() {
    const DIGITS: usize = 1000;
    
    let mut fib = Fib::new();
    let mut index = 2;
    loop {
        let mut next = fib.next().unwrap();
        // Getting digits
        let mut dig_num = 0;
        while !next.is_zero() {
            dig_num += 1;
            next /= 10u32;
        }
        if dig_num >= DIGITS {
            break;
        }
        index += 1;
    };

    println!("{}", index);
}