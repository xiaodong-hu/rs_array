use crate::{array_basic::*, scalar::Scalar};
use std::{
    ops::{Add, Mul, Sub},
    simd::{SimdFloat, SimdInt},
};

trait SimdSpecialized {
    type SimdVecType;
    const LANES: usize;

    fn load(data: &[Self]) -> Self::SimdVecType
    where
        Self: Sized;
    fn multiply_add(
        a: Self::SimdVecType,
        b: Self::SimdVecType,
        c: Self::SimdVecType,
    ) -> Self::SimdVecType
    where
        Self: Sized;
    fn sum(vec: Self::SimdVecType) -> Self
    where
        Self: Sized;
    fn splat(value: Self) -> Self::SimdVecType
    where
        Self: Sized;
}

impl SimdSpecialized for f32 {
    type SimdVecType = std::simd::f32x4;
    const LANES: usize = 4;

    fn load(data: &[Self]) -> Self::SimdVecType {
        std::simd::f32x4::from_slice(data)
    }
    fn multiply_add(
        a: Self::SimdVecType,
        b: Self::SimdVecType,
        c: Self::SimdVecType,
    ) -> Self::SimdVecType {
        a * b + c
    }
    fn sum(vec: Self::SimdVecType) -> Self {
        vec.reduce_sum()
    }
    fn splat(value: Self) -> Self::SimdVecType {
        std::simd::f32x4::splat(value)
    }
}

impl SimdSpecialized for f64 {
    type SimdVecType = std::simd::f64x4;
    const LANES: usize = 4;

    fn load(data: &[Self]) -> Self::SimdVecType {
        std::simd::f64x4::from_slice(data)
    }
    fn multiply_add(
        a: Self::SimdVecType,
        b: Self::SimdVecType,
        c: Self::SimdVecType,
    ) -> Self::SimdVecType {
        a * b + c
    }
    fn sum(vec: Self::SimdVecType) -> Self {
        vec.reduce_sum()
    }
    fn splat(value: Self) -> Self::SimdVecType {
        std::simd::f64x4::splat(value)
    }
}

impl SimdSpecialized for i32 {
    type SimdVecType = std::simd::i32x4;
    const LANES: usize = 4;

    fn load(data: &[Self]) -> Self::SimdVecType {
        std::simd::i32x4::from_slice(data)
    }
    fn multiply_add(
        a: Self::SimdVecType,
        b: Self::SimdVecType,
        c: Self::SimdVecType,
    ) -> Self::SimdVecType {
        a * b + c
    }
    fn sum(vec: Self::SimdVecType) -> Self {
        vec.reduce_sum()
    }
    fn splat(value: Self) -> Self::SimdVecType {
        std::simd::i32x4::splat(value)
    }
}

impl<T: Scalar + SimdSpecialized> Array<T> {
    pub fn mul_standard_simd(&self, rhs: &Array<T>) -> Array<T> {
        assert_eq!(self.shape.len(), 2, "Left operand must be a 2D matrix");
        assert_eq!(rhs.shape.len(), 2, "Right operand must be a 2D matrix");
        assert_eq!(
            self.shape[1], rhs.shape[0],
            "Dimensions mismatch for matrix multiplication"
        );

        let rows = self.shape[0];
        let cols = rhs.shape[1];
        let mut product_data = Vec::with_capacity(rows * cols);

        for i in 0..rows {
            for j in 0..cols {
                let mut sum = T::splat(T::Zero);

                for k in (0..self.shape[1]).step_by(T::LANES) {
                    if k + T::LANES <= self.shape[1] {
                        let left_slice =
                            &self.data[self.calculate_data_index_from_array_indices(vec![i, k])..];
                        let right_slice =
                            &rhs.data[rhs.calculate_data_index_from_array_indices(vec![k, j])..];

                        let left_vec = T::load(left_slice);
                        let right_vec = T::load(right_slice);

                        sum = T::multiply_add(left_vec, right_vec, sum);
                    } else {
                        // Handle the remaining elements (non-SIMD part)
                        for k_remain in k..self.shape[1] {
                            let left_index =
                                self.calculate_data_index_from_array_indices(vec![i, k_remain]);
                            let right_index =
                                rhs.calculate_data_index_from_array_indices(vec![k_remain, j]);
                            sum = T::multiply_add(
                                T::splat(self.data[left_index].clone()),
                                T::splat(rhs.data[right_index].clone()),
                                sum,
                            );
                        }
                        break;
                    }
                }
                product_data.push(T::sum(sum));
            }
        }

        Array {
            data: product_data,
            shape: vec![rows, cols],
        }
    }
}
