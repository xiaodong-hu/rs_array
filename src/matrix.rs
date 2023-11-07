use crate::{array_general::*, scalar::Scalar};
// use crate::scalar::*;
// use num_traits::*;

/* macros */
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
                shape: vec![col_len, row_len],
            };
            println!("{}",array);
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
                shape: vec![col_len, row_len],
            };
            println!("{}",array);
            array.transpose()
        }
    };
}

/* implementations for array of two dimensions */
impl<T: Scalar> Array<T> {
    /// transpose of **two-dimensional** `Array<T>`
    pub fn transpose(&self) -> Array<T> {
        // Ensure the array is two-dimensional
        assert_eq!(self.shape.len(), 2, "Array must be 2-dimensional");

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

    /// SVD
    pub fn svd(&self) -> (Array<T>, Array<T>, Array<T>) {
        todo!()
    }
}
