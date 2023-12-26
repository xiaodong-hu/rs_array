use crate::scalar::Scalar;
// use num_traits::*;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub enum DataOrder {
    RowMajor,
    ColMajor,
}
impl DataOrder {
    pub fn alternate(&self) -> Self {
        match self {
            DataOrder::RowMajor => DataOrder::ColMajor,
            DataOrder::ColMajor => DataOrder::RowMajor,
        }
    }
}

// ### General Array Container of Arbitrary Dimension
/// of generic scalar datatype `data: Vec<T: Scalar>` and dimensions `shape: Vec<usize>`.
///
/// We use `Scalar` type here for generic usage, including float, string, and symbolic variables. And we use `Vec<usize>` for the shape rather than `[usize; N]` with compile-time const generics `N` simply because we want to support dynamic dimensionality, such as fast `reshape` method.
///
/// The Array is set to be **column-major** by default, meaning that the data of array-position `[i,j,k,...]` is stored at `data_index = i + j * i_length + k * j_length * i_length + ...`. This is the same as the convention of `numpy`, `Fortran`, and `julia`.
#[derive(Debug, Clone)]
pub struct Array<T: Scalar, const D: usize> {
    pub data: Vec<T>,
    pub shape: [usize; D],
    pub data_order: DataOrder,
}

/* Method implementations for general dimensions */
impl<T: Scalar, const D: usize> Array<T, D> {
    /// Reshape by clone
    pub fn reshape(&self, new_shape: [usize; D]) -> Self {
        assert!(new_shape.iter().product::<usize>() == self.data.len());
        Array {
            data: self.data.clone(),
            shape: new_shape,
            data_order: self.data_order,
        }
    }
    /// Reshape in-place
    pub fn reshape_inplace(self, new_shape: [usize; D]) -> Self {
        assert!(new_shape.iter().product::<usize>() == self.data.len());
        Array {
            data: self.data,
            shape: new_shape,
            data_order: self.data_order,
        }
    }
    /// Element-wisely Broadcast with closures by clone
    ///
    /// Here we use generics to allow type conversion for data. Example usage: `m.map(|x| x as f64)` for `m` of `Array<i32>`
    pub fn map<U: Scalar, F: Fn(T) -> U>(&self, func: F) -> Array<U, D> {
        Array {
            data: self.data.clone().into_iter().map(func).collect::<Vec<U>>(),
            shape: self.shape,
            data_order: self.data_order,
        }
    }
    /// Element-wisely Broadcast with closures in-place
    pub fn map_inplace<U: Scalar, F: Fn(T) -> U>(self, func: F) -> Array<U, D> {
        Array {
            data: self.data.into_iter().map(func).collect::<Vec<U>>(),
            shape: self.shape,
            data_order: self.data_order,
        }
    }
    /// Get the maximum string length for the element: also work for future symbolic variables!
    pub fn get_element_length_and_interval(
        &self,
        decimal_length: usize,
        element_interval: usize,
    ) -> usize {
        let width = self
            .data
            .iter()
            .map(|x| format!("{:>.decimal_length$}", x, decimal_length = decimal_length).len())
            .max()
            .unwrap();

        width + element_interval
    }
}

/* Trait implementations for general dimensions */
/// array addition by clone, namely `&A+&B` create a new array: both `A` and `B` are not consumed
impl<T: Scalar + Add<Output = T>, const D: usize> Add for &Array<T, D> {
    type Output = Array<T, D>;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            self.shape.iter().eq(rhs.shape.iter()),
            "Check Input: Dimension Mismatch!"
        );
        let res_data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect();
        Array {
            data: res_data,
            shape: self.shape,
            data_order: self.data_order,
        }
    }
}
/// array addition in-place, namely `A+B` consume both `A` and `B` and the resultant is stored in `A`
impl<T: Scalar + Add<Output = T>, const D: usize> Add for Array<T, D> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        assert!(self.shape == rhs.shape, "Dimension Mismatch!");
        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
            *a = a.clone() + b.clone();
        }
        self
    }
}
/// array substraction by clone, namely `&A-&B` create a new array: both `A` and `B` are not consumed
impl<T: Scalar + Sub<Output = T>, const D: usize> Sub for &Array<T, D> {
    type Output = Array<T, D>;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.shape.iter().eq(rhs.shape.iter()),
            "Check Input: Dimension Mismatch!"
        );
        let res_data = self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect();
        Array {
            data: res_data,
            shape: self.shape,
            data_order: self.data_order,
        }
    }
}
/// array subtraction in-place, namely `A+B` consume both `A` and `B` and the resultant is stored in `A`
impl<T: Scalar + Sub<Output = T>, const D: usize> Sub for Array<T, D> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        assert!(self.shape == rhs.shape, "Dimension Mismatch!");
        for (a, b) in self.data.iter_mut().zip(rhs.data.iter()) {
            *a = a.clone() - b.clone();
        }
        self
    }
}

/* Macros */
/// Calculate the `data_index` of the one-dimensional raw data of `Array` for the given multi-dimensional indices `[i,j,k,...]`.
///
/// The **column major** relation is `data_index = i + j * i_length + k * j_length * i_length + ...`
#[macro_export]
macro_rules! calculate_data_index {
    ($array:expr, $indices:expr) => {{
        assert!($indices.len() == $array.shape.len()); // ensure the number of indices is equal to the dimension of array
        assert!($indices.len() > 0);

        let mut data_index = 0;
        let mut product = 1;
        match $indices.len() {
            1 => {
                data_index = $indices[0];
            }
            2 => {
                data_index = $indices[0] + $indices[1] * $array.shape[0];
            }
            _ => {
                for (current_dim, &current_index) in $indices.iter().enumerate() {
                    data_index += current_index * product;
                    product *= $array.shape[current_dim];
                }
                assert!(data_index < $array.data.len()); // ensure the index is within the range of data
            }
        }
        data_index
    }};
}

/// Initialize multi-dimensional array with every element zero or one
///
/// Example usage: `zeros!(f64,2,3,4)`
#[macro_export]
macro_rules! zeros {
    ($type:ty, $($dim:expr),+) => {
        {
            use crate::scalar::Scalar;
            let shape = vec![$($dim),+];
            let data_length = shape.iter().product();
            Array {
                data: vec![<$type as Scalar>::Zero; data_length], // syntax <Type as Trait> is used for specifying trait bounds
                shape,
                data_order: DataOrder::ColMajor,
            }
        }
    };
}
#[macro_export]
macro_rules! ones {
    ($type:ty, $($dim:expr),+) => {
        {
            use crate::scalar::Scalar;
            use crate::array_basic::*;
            let shape = vec![$($dim),+];
            let data_length = shape.iter().product();
            Array {
                data: vec![<$type as Scalar>::One; data_length], // syntax <Type as Trait> is used for specifying trait bounds
                shape,
                data_order: DataOrder::ColMajor,
            }
        }
    };
}
/// Initialize multi-dimensional array with random numbers
///
/// Example usage: `randn!(f64,2,3,4)`
#[macro_export]
macro_rules! randn {
    ($type:ty, $($dim:expr),+) => {
        {
            use rand::Rng;
            use crate::array_basic::*;
            let mut rng = rand::thread_rng();
            let shape = [$($dim),+];
            let data_length = shape.iter().product();
            Array {
                data: (0..data_length).map(|_| rng.gen::<$type>()).collect(),
                shape,
                data_order: DataOrder::ColMajor,
            }
        }
    };
}

/// Reshape the multi-dimensional array
///
/// Example usage: `reshape!(m,1,6)` for `m` of `Array<f64>` with shape `[2,3]`
#[macro_export]
macro_rules! reshape {
    ($array:expr, $($dim:expr),+) => {
        {
            $array.reshape_inplace([$($dim),+])
        }
    };
}
