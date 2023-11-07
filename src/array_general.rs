use crate::scalar::Scalar;
// use num_traits::*;
use std::ops::{Add, Mul, Sub};
use std::ops::{Index, IndexMut};

// #[derive(Debug)]
// pub struct Shape<const N: usize>([usize; N]);

/// ### Array Type of General Dimension
/// with generic scalar datatype `data: Vec<T: Scalar>` and a runtime dimension of the array `shape: Vec<usize>`.
#[derive(Debug)]
pub struct Array<T: Scalar> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

/* Method implementations for general dimensions */
impl<T: Scalar> Array<T> {
    /// Calculate the `data_index` of the one-dimensional raw data of `Array` for the given multi-dimensional indices `[i,j,k,...]`.
    ///
    /// The **(column major)** relation is `data_index = i + j * i_length + k * j_length * i_length + ...`
    ///
    /// This method is crucial for display
    pub fn calculate_data_index_from_array_indices(&self, indices: Vec<usize>) -> usize {
        let mut data_index = 0;
        for (current_dim, current_index) in indices.iter().enumerate() {
            data_index += current_index * {
                self.shape[0..current_dim].iter().fold(1, |acc, x| acc * x) // cumulative sum
            }
        }
        data_index
    }
    /// Reshape the array in-place (with consumption).
    ///
    /// No clone of data occurs here: only parameter `shape` is changed.
    /// Error if the shape does not match with data
    pub fn reshape(self, new_shape: Vec<usize>) -> Self {
        assert!(new_shape.iter().product::<usize>() == self.data.len());
        Array {
            data: self.data,
            shape: new_shape,
        }
    }
    /// Broadcast closures element-wisely (with consumption).
    ///
    /// Here we extend the type condition so that the resultant `Array<U>` can be of different type of the original one `Array<T>`. Example usage:
    /// ```rust
    /// m = matrix![1 2 3; 4 5 6];
    /// m.map(|x| x as f64);
    /// ```
    pub fn map<U: Scalar, F: Fn(T) -> U>(self, func: F) -> Array<U> {
        Array {
            data: self.data.into_iter().map(func).collect::<Vec<U>>(),
            shape: self.shape,
        }
    }
}

/* Trait implementations for general dimensions */
impl<T: Scalar + Add<Output = T>> Add for Array<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(
            self.shape.iter().eq(rhs.shape.iter()),
            "Check Input: Dimension Mismatch!"
        );
        let data_length = self.shape.iter().product::<usize>();
        let mut res_data = Vec::<T>::with_capacity(data_length);
        for i in 0..data_length {
            res_data.push(self.data[i].clone() + rhs.data[i].clone())
        }
        Array {
            data: res_data,
            shape: self.shape,
        }
    }
}
impl<T: Scalar + Sub<Output = T>> Sub for Array<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.shape.iter().eq(rhs.shape.iter()),
            "Check Input: Dimension Mismatch!"
        );
        let data_length = self.shape.iter().product::<usize>();
        let mut res_data = Vec::<T>::with_capacity(data_length);
        for i in 0..data_length {
            res_data.push(self.data[i].clone() - rhs.data[i].clone())
        }
        Array {
            data: res_data,
            shape: self.shape,
        }
    }
}

impl<T: Scalar + Mul<Output = T>> Mul<T> for Array<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Array {
            data: self
                .data
                .clone()
                .iter()
                .map(|x| x.to_owned() * rhs.clone())
                .collect(),
            shape: self.shape,
        }
    }
}

/* Macros */
/// Initialize multi-dimensional array with every element zero
///
/// Example usage: `zeros!(f64,2,3,4)`
#[macro_export]
macro_rules! zeros {
    ($type:ty, $($dim:expr),+) => {
        {
            use crate::scalar::Scalar;
            let shape = vec![$($dim),+];
            let total_size = shape.iter().product();
            Array {
                data: vec![<$type as Scalar>::Zero; total_size], // syntax <Type as Trait> is used for specifying trait bounds
                shape,
            }
        }
    };
}
/// Initialize multi-dimensional array with every element one
///
/// Example usage: `ones!(i32,2,3,4)`
#[macro_export]
macro_rules! ones {
    ($type:ty, $($dim:expr),+) => {
        {
            use crate::scalar::Scalar;
            let shape = vec![$($dim),+];
            let total_size = shape.iter().product();
            Array {
                data: vec![<$type as Scalar>::One; total_size], // syntax <Type as Trait> is used for specifying trait bounds
                shape,
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
            let mut rng = rand::thread_rng();
            let shape = vec![$($dim),+];
            let total_size = shape.iter().product();
            Array {
                data: (0..total_size).map(|_| rng.gen::<$type>()).collect(),
                shape,
            }
        }
    };
}

/// Reshape the multi-dimensional array
///
/// Example usage:
/// ```rust
/// let m = Array {
///     data: vec![1, 2, 3, 4, 5, 6],
///     shape: [2, 3].to_vec(),
/// };
/// let m_new = reshape!(m,(1,6))
/// ```
#[macro_export]
macro_rules! reshape {
    ($array:expr, $($dim:expr),+) => {
        {
            $array.reshape(vec![$($dim),+])
        }
    };
}
