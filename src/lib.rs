#![feature(portable_simd)]
#![feature(test)]

extern crate test;

mod array_basic;
mod array_display;
mod matrix;
mod matrix_simd;
mod scalar;

use crate::array_basic::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let m = matrix![1 2; 3 4; 5 6];
        println!("{m}");
        println!("{}", reshape!(m, 1, 6)); // m is consumed here

        let m = matrix![1 2 3; 4 5 6];
        println!("{}", m.map(|x| x * 2)); // m is NOT consumed after map
        println!("{m}");

        let m = randn!(f64, 4, 3, 2); // multi-dimensional array
        println!("{m}");
    }

    fn matrix_multiplication_test(dim: usize) -> Array<f64> {
        let m1 = randn!(f64, dim, dim);
        let m2 = randn!(f64, dim, dim);
        m1.mul_standard(&m2)
    }

    use test::Bencher;
    #[bench]
    fn matrix_multiplication_bench(b: &mut Bencher) {
        b.iter(|| matrix_multiplication_test(5));
    }
}
