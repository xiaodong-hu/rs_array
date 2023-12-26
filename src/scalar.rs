// use num_traits::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

/// **Minimal** trait bound for the datatype to be stored in `Array<T>`
///
/// Note: `Clone` is OK here since *No Extra Overhead* will occurs for numerical types that satisfy `Copy` trait
pub trait Scalar: Clone + Display + Sized {}
// impl the trait bound `Scalar` for all built-in numeric types
macro_rules! impl_Scalar_for_types {
    ($($type:ty) *) => ($( // `ty` is specific for the fragment specifier `type`
        impl Scalar for $type {}
    )*)
}
impl_Scalar_for_types!(usize isize u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);

/// **Minimal** trait bound to support basic linear-algebra operations for `Array<T>`
pub trait Arithmetic<T>: Add<Output = T> + Mul<Output = T> + Sized {
    const ZERO: Self;
}
// impl the trait bound `Arithmetic` for all built-in numeric types
macro_rules! impl_Arithmetic_for_types {
    ($($type:ty) *) => ($( // `ty` is specific for the fragment specifier `type`
        impl Arithmetic<$type> for $type {
            const ZERO: Self = 0 as $type; // coerce type conversion
        }
    )*)
}
impl_Arithmetic_for_types!(usize isize u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);
