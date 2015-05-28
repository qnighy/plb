extern crate nalgebra;

use std::env;
use nalgebra::*;

fn matgen(n : usize) -> DMat<f64> {
    let tmp = 1.0 / (n as f64) / (n as f64);
    DMat::from_fn(n, n, |i, j| {
        let i = i as i32;
        let j = j as i32;
        tmp * ((i - j) as f64) * ((i + j) as f64)
    })
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let n : usize =
        if args.len() >= 2 {
            args[1].trim().parse().ok().expect("Failed to parse argument")
        } else {
            100
        };

    let n = n / 2 * 2;
    let a = matgen(n);
    let b = matgen(n);
    let c = a * b;
    println!("{}", c[(n/2, n/2)]);
}
