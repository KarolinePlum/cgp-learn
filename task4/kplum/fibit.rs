use std::iter::Iterator;

fn main() {
    for i in FibIt::new().take(20) {
        println!("{}", i);
    }
}

struct FibIt {
    fn_minus_1: u32,
    fn_minus_2: u32,
}

impl Iterator for FibIt {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let f_n = self.fn_minus_1 + self.fn_minus_2;
        self.fn_minus_2 = self.fn_minus_1;
        self.fn_minus_1 = f_n;
        Some(f_n)
    }
}

impl FibIt {
    fn new() -> FibIt {
        FibIt {
            fn_minus_1: 1,
            fn_minus_2: 0,
        }
    }
}
