struct Fib {
    cur: usize,
    a: usize,
    b: usize,
    max: usize,
}

impl Fib {
    fn new(max: usize) -> Fib {
        Fib { cur: 1, a: 1, b: 0, max}
    }
}

impl Iterator for Fib {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        {
            self.cur = self.a + self.b;
            self.b = self.a;
            self.a = self.cur;
        }
        
        if self.cur < self.max {
            Some(self.cur)
        }
        else {
            None
        }
    }
}


fn main() {
    let fib = Fib::new(4_000_000);
    let result: usize = fib.filter(|x| x % 2 == 0).sum();

    println!("{}", result)
}