use crate::{
    array_basic::*,
    calculate_data_index,
    scalar::{Arithmetic, Scalar},
};
use num_traits::*;
use std::ops::{Add, Mul, Sub};

/* Macros */
#[macro_export]
macro_rules! matrix {
    ( $( $( $x:expr ),+ );+ $(;)? ) => {
        {
            use crate::array_basic::*;
            let row_len = [ $( [ $($x),+ ] ),+ ].len();
            let col_len_list = [$([ $($x),+ ].len()),*];
            let col_len = col_len_list[0];
            let col_len_list_indicator = col_len_list.iter().all(|&l| l==col_len);
            assert!(col_len_list_indicator, "Check Input: the column dimensions do not match for each row!");
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
            use crate::array_basic::*;
            let row_len = [ $( [ $($x),+ ] ),+ ].len();
            let col_len_list = [$([ $($x),+ ].len()),*];
            let col_len = col_len_list[0];
            let col_len_list_indicator = col_len_list.iter().all(|&l| l==col_len);
            assert!(col_len_list_indicator, "Check Input: the column dimensions do not match for each row!");
            let data = vec![$( $($x),+ ),*];
            let array = Array {
                data,
                shape: vec![col_len,row_len],
            };
            array.transpose()
        }
    };
}

/* Implementations for array of two dimensions, i.e., Matrices */
impl<T: Scalar> Array<T> {
    /// check matrix dimension
    #[inline]
    fn matrix_dimension_check(&self) {
        assert_eq!(self.shape.len(), 2, "Check Input: Dimension Mismatch!");
    }

    /// transpose of **two-dimensional** `Array<T>`
    pub fn transpose(&self) -> Array<T> {
        // ensure the array is two-dimensional
        self.matrix_dimension_check();
        let mut res_data = Vec::<T>::with_capacity(self.data.len());
        for j in 0..self.shape[1] {
            for i in 0..self.shape[0] {
                let index = calculate_data_index!(self, [i, j]);
                res_data.push(self.data[index].clone());
            }
        }
        Array {
            data: res_data,
            shape: vec![self.shape[1], self.shape[0]],
        }
    }
}

// Mul<Output = T> + Add<Output = T>
impl<T: Scalar + Arithmetic<T>> Array<T> {
    /// check dimension and multiplication relevant length
    #[inline]
    fn matrix_multiplication_check(lhs: &Array<T>, rhs: &Array<T>) {
        lhs.matrix_dimension_check();
        rhs.matrix_dimension_check();
        assert!(lhs.shape[1] == rhs.shape[0]);
    }

    /// Naive `O(n^3)` multiplication
    pub fn mul_naive(&self, rhs: &Array<T>) -> Array<T> {
        Array::matrix_multiplication_check(self, rhs);

        let (res_row, res_col) = (self.shape[0], self.shape[1]);
        let mut res_data = Vec::with_capacity(res_row * res_col);

        // let self_transpose = self.transpose();

        // switch the order of the loops to improve cache hit rate
        for j in 0..res_col {
            for i in 0..res_row {
                let mut sum = T::ZERO;
                for k in 0..self.shape[1] {
                    let lhs_index = calculate_data_index!(self, [i, k]);
                    let rhs_index = calculate_data_index!(rhs, [k, j]);
                    sum = sum + self.data[lhs_index].clone() * rhs.data[rhs_index].clone();
                    // it is OK to use `.clone()` here when datatype support copy trait: the compiler is smart enough to replace with stack-copy and skip the overhead
                }
                res_data.push(sum);
            }
        }
        Array {
            data: res_data,
            shape: vec![res_row, res_col],
        }
    }

    /// Strassen algorithm of matrix multiplcation, complexity `O(n^{log_2 7})=O(n^{2.807})`
    pub fn mul_strassen(&self, rhs: &Array<T>) -> Array<T> {
        Array::matrix_multiplication_check(self, rhs);
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
