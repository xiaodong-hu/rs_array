use crate::{array_basic::*, scalar::Scalar};
use std::ops::{Add, Mul, Sub};
// use crate::scalar::*;
// use num_traits::*;

/* Macros */
#[macro_export]
macro_rules! matrix {
    ( $( $( $x:expr ),+ );+ $(;)? ) => {
        {
            let row_len = [ $( [ $($x),+ ] ),+ ].len();
            let col_len_vec = [$([ $($x),+ ].len()),*];
            let col_len = col_len_vec[0];
            let col_len_vec_indicator = col_len_vec.iter().all(|&l| l==col_len);
            assert!(col_len_vec_indicator, "Check Input: the cols dimensions do not equal for every row!");
            let data = vec![$( $($x),+ ),*];
            let array = Array {
                data,
                shape: vec![col_len,row_len],
            };
            array.transpose()
        }
    };
    ( $( $( $x:expr ) + );+ $(;)? ) => {
        {
            let row_len = [ $( [ $($x),+ ] ),+ ].len();
            let col_len_vec = [$([ $($x),+ ].len()),*];
            let col_len = col_len_vec[0];
            let col_len_vec_indicator = col_len_vec.iter().all(|&l| l==col_len);
            assert!(col_len_vec_indicator, "Check Input: the cols dimensions do not equal for every row!");
            let data = vec![$( $($x),+ ),*];
            let array = Array {
                data,
                shape: vec![col_len,row_len],
            };
            array.transpose()
        }
    };
}

/* Implementations for array of two dimensions */
impl<T: Scalar> Array<T> {
    /// transpose of **two-dimensional** `Array<T>`
    pub fn transpose(&self) -> Array<T> {
        // ensure the array is two-dimensional
        assert_eq!(
            self.shape.len(),
            2,
            "transpose is only defined for 2D matrix"
        );

        let rows = self.shape[0];
        let cols = self.shape[1];
        let mut transposed_data = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                let index = self.calculate_data_index_from_array_indices([i, j].to_vec());
                transposed_data.push(self.data[index].clone());
            }
        }
        Array {
            data: transposed_data,
            shape: vec![cols, rows],
        }
    }
}

impl<T: Scalar + Mul<Output = T> + Add<Output = T>> Array<T> {
    /// Standard `O(n^3)` multiplication
    pub fn mul_standard(&self, rhs: &Array<T>) -> Array<T> {
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
                let mut sum = T::Zero;
                for k in 0..self.shape[1] {
                    let left_index = self.calculate_data_index_from_array_indices(vec![i, k]);
                    let right_index = rhs.calculate_data_index_from_array_indices(vec![k, j]);
                    sum = sum + self.data[left_index].clone() * rhs.data[right_index].clone();
                }
                product_data.push(sum);
            }
        }
        Array {
            data: product_data,
            shape: vec![rows, cols],
        }
    }

    /// Strassen algorithm of matrix multiplcation, complexity `O(n^{log_2 7})=O(n^{2.807})`
    pub fn mul_strassen(&self, rhs: Array<T>) -> Self {
        todo!()
    }

    /// SVD
    pub fn svd(&self) -> (Array<T>, Array<T>, Array<T>) {
        todo!()
    }

    /// QR
    pub fn qr(&self) -> (Array<T>, Array<T>) {
        todo!()
    }
}
