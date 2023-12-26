#![feature(portable_simd)]

// use std::simd::{f64x64, f64x8};

use rs_bench::*;

mod array_basic;
mod array_display;
mod matrix;
// mod matrix_simd;
mod array_slice;
mod scalar;

// use rayon::prelude::*;

fn dot_simd(lhs: &[f64], rhs: &[f64]) -> f64 {
    use std::simd::{num::SimdFloat, Simd};
    let lhs_len = lhs.len();
    let rhs_len = rhs.len();
    assert_eq!(lhs_len, rhs_len);
    const SIMD_WIDTH: usize = 4; // AVX2 supports 256 bits, so 8 f64 values

    // let mut res_simd: Simd<f64, SIMD_WIDTH> = Simd::splat(0.0 as f64);
    // // Process in chunks of SIMD_WIDTH
    // let mut index: usize = 0; // chunk index
    let residule_len = lhs_len % SIMD_WIDTH;
    let mut res = lhs
        .chunks_exact(SIMD_WIDTH)
        .map(Simd::from_slice)
        .zip(rhs.chunks_exact(SIMD_WIDTH).map(Simd::from_slice))
        .map(|(lhs_simd, rhs_simd)| {
            lhs_simd * rhs_simd // simd dot product, still resulting in a simd vector
        })
        .sum::<Simd<_, SIMD_WIDTH>>()
        .reduce_sum(); // horizontal sum of the simd vector

    if residule_len == 0 {
        res
    } else {
        lhs.iter()
            .skip(lhs_len - residule_len)
            .zip(rhs.iter().skip(rhs_len - residule_len))
            .map(|(a, b)| a * b)
            .sum::<f64>()
            + res
    }
}

fn main() {
    let m1 = randn!(f64, 25);
    let m2 = randn!(f64, 25);

    let m1 = m1.data;
    let m2 = m2.data;

    time_block![{
        let res = m1.iter().zip(m2.iter()).map(|(a, b)| a * b).sum::<f64>();
        // println!("{res}");
    }];

    time_block![{
        let res = dot_simd(&m1, &m2);
        // println!("{res}")
    }];
    // println!("test");
}
