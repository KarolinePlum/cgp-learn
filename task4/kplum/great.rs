use std::ops::{Mul, Add};

fn wow<T: Mul + Add + Copy>(x: T, y: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (x + y, x * y)
}

fn main() {
    println!("{:?}", wow(3, 4));
}
