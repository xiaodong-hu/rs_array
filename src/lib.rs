#![feature(portable_simd)]

use rs_bench::*;

mod array_basic;
mod array_display;
mod matrix;
// mod matrix_simd;
mod array_slice;
mod scalar;

// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let m = matrix![1 2; 3 4; 5 6];

        dbg!(calculate_data_index!(m, [2, 1]));
        println!("{m}");
        println!("{}", reshape!(m, 1, 6)); // m is consumed here

        time_block![{
            let m1 = matrix![1 2 3; 4 5 6];
            let m2 = m1.clone();

            println!("{}", &m1 - &m2); // m1 and m2 are NOT consumed after subtraction
            println!("{}", m1 - m2); // m1 and m2 are consumed after subtraction (and is stored in `m1`)
        }];

        time_block![
            {
                let m = matrix![1 2 3; 4 5 6];
                println!("{}", m.map(|x| x - 3)); // m is NOT consumed after map
                println!("{m}");
            },
            "matrix transformation"
        ];

        let m1 = matrix![1 2 3; 4 5 6];
        let m2 = matrix![1 2; 3 4; 5 6];

        time_block![{ println!("{}", m1.mul_naive(&m2)) }];

        dbg!("test");
        // println!("{}", m.mul_naive(&m.transpose()));
    }

    // fn matrix_multiplication_test(dim: usize) {
    // let m1 = matrix![1 2 3; 4 5 6];
    // let m2 = randn!(f64, dim, dim);
    // m1.mul_naive(&m1);
    // }

    // use std::simd::{f64x4, SimdFloat};
    // fn dot_simd(vec1: &[f64], vec2: &[f64]) -> f64 {
    //     assert_eq!(vec1.len(), vec2.len());

    //     let len = vec1.len();
    //     let full_chunks = len / f64x4::LANES; // Number of full SIMD chunks
    //     let remainder = len % f64x4::LANES; // Number of elements in the partial chunk

    //     let mut sum = f64x4::splat(0.0);

    //     // Process full SIMD chunks
    //     for i in 0..full_chunks {
    //         let base_index = i * f64x4::LANES;
    //         let chunk1 = f64x4::from_slice(&vec1[base_index..base_index + f64x4::LANES]);
    //         let chunk2 = f64x4::from_slice(&vec2[base_index..base_index + f64x4::LANES]);
    //         sum += chunk1 * chunk2;
    //     }

    //     // Process remaining elements
    //     let mut remainder_sum = 0.0;
    //     for i in full_chunks * f64x4::LANES..len {
    //         remainder_sum += vec1[i] * vec2[i];
    //     }

    //     sum.reduce_sum() + remainder_sum
    // }

    // fn dot_normal(vec1: &[f64], vec2: &[f64]) -> f64 {
    //     assert_eq!(vec1.len(), vec2.len());
    //     vec1.iter().zip(vec2).fold(0.0, |acc, x| acc + x.0 * x.1)
    // }

    // #[test]
    // fn dot_test() {
    //     const N: usize = 1000000; // take prominent effect at N~100000
    //     let v1 = randn!(f64, N);
    //     let v1 = v1.data.as_slice();
    //     let v2 = randn!(f64, N);
    //     let v2 = v2.data.as_slice();

    //     use std::time::Instant;
    //     let now = Instant::now();
    //     println!("simd dot: {}", dot_simd(v1, v2));
    //     let elapsed = now.elapsed();
    //     println!("Elapsed: {:.2?}", elapsed);

    //     let now = Instant::now();
    //     println!("normal dot: {}", dot_normal(v1, v2));
    //     let elapsed = now.elapsed();
    //     println!("Elapsed: {:.2?}", elapsed);
    // }

    // use test::Bencher;
    // #[bench]
    // fn matrix_multiplication_bench(b: &mut Bencher) {
    //     b.iter(|| matrix_multiplication_test(5));
    // }
}
