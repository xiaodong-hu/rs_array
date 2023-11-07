// use num_traits::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};

// pub trait Arithmetic<Rhs = Self, Output = Self>:
//     Add<Rhs, Output = Self>
//     + Mul<Rhs, Output = Self>
//     + Sub<Rhs, Output = Self>
//     + Div<Rhs, Output = Self>
// {
// }
// impl<T, Rhs, Output> Arithmetic<Rhs, Output> for T where
//     T: Add<Rhs, Output = Self>
//         + Sub<Rhs, Output = Self>
//         + Mul<Rhs, Output = Self>
//         + Div<Rhs, Output = Self>
// {
// }

// pub trait Num: Display + PartialEq + PartialOrd + Arithmetic {}

// impl Add for i32 {
//     type Output = f64;
//     fn mul(self, rhs: Self) -> Self::Output {
//         (self as f64) * rhs
//     }
// }

/// ### Trait Bound for the Datatype to be Stored in Array
pub trait Scalar: Clone + Display + PartialEq + PartialOrd {
    const Zero: Self;
    const One: Self;
}

// impl the trait bound `Scalar` for all built-in numeric types
macro_rules! multiple_type_trait_implementation {
    ($name:ident for $($type:ty)*) => ($( // `ty` is specific for the fragment specifier `type`
        impl $name for $type {
            const Zero: Self = 0 as $type; // coerce type conversion
            const One: Self = 1 as $type; // coerce type conversion
        }
    )*)
}
// multiple_type_trait_implementation!(Num for usize isize u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);
multiple_type_trait_implementation!(Scalar for usize isize u8 u16 u32 u64 i8 i16 i32 i64 f32 f64);
