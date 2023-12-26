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
                data_order: DataOrder::RowMajor,
            };
            array
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
                shape: [col_len,row_len],
                data_order: DataOrder::RowMajor,
            };
            array
        }
    };
}

/* Implementations for array of two dimensions, i.e., Matrices */
impl<T: Scalar> Array<T, 2> {
    /// transpose of **two-dimensional** `Array<T>`
    pub fn transpose(&self) -> Array<T, 2> {
        // let mut res_data = Vec::<T>::with_capacity(self.data.len());
        let mut res_data = self.data.clone();
        let (rows, cols) = (self.shape[0], self.shape[1]);

        // for i in 0..rows {
        //     for j in 0..cols {
        //         let original_index = i + j * rows;
        //         let transposed_index = j + i * cols;
        //         res_data[transposed_index] = self.data[original_index].clone();
        //     }
        // }

        // for i in 0..rows {
        //     for j in 0..cols {
        //         let index = calculate_data_index!(self, [i, j]);
        //         res_data.push(self.data[index].clone());
        //     }
        // }
        Array {
            data: self.data.clone(),
            shape: [cols, rows],
            data_order: self.data_order.alternate(),
        }
    }
}

// Mul<Output = T> + Add<Output = T>
impl<T: Scalar + Arithmetic<T>> Array<T, 2> {
    /// check dimension and multiplication relevant length
    #[inline]
    fn matrix_multiplication_check(lhs: &Array<T, 2>, rhs: &Array<T, 2>) {
        assert!(lhs.shape[1] == rhs.shape[0]);
    }

    /// Naive `O(n^3)` multiplication
    pub fn mul_naive(&self, rhs: &Array<T, 2>) -> Array<T, 2> {
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
            shape: [res_row, res_col],
            data_order: self.data_order,
        }
    }

    /// Strassen algorithm of matrix multiplcation, complexity `O(n^{log_2 7})=O(n^{2.807})`
    pub fn mul_strassen(&self, rhs: &Array<T, 2>) -> Array<T, 2> {
        Array::matrix_multiplication_check(self, rhs);
        todo!()
    }

    /// SVD
    pub fn svd(&self) -> (Array<T, 2>, Array<T, 2>, Array<T, 2>) {
        todo!()
    }

    /// QR
    pub fn qr(&self) -> (Array<T, 2>, Array<T, 2>) {
        todo!()
    }
}
