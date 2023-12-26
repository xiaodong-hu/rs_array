use crate::array_basic::*;
use crate::scalar::*;

use std::ops::{Index, IndexMut, Range, RangeFull};

pub enum Axes {
    FullAxis,            // the full axis
    Range(usize, usize), // a range (start, end) in the axis
    Slot(usize),         // a single slot in the axis
}

impl<T: Scalar> Array<T> {
    #[inline]
    fn check_slice_legitimacy(&self, slices: &[Axes]) {
        assert_eq!(self.shape.len(), slices.len());
    }

    pub fn slice_view(&self, slices: &[Axes]) -> &Array<T> {
        self.check_slice_legitimacy(slices);

        todo!()
    }

    pub fn slice(&mut self, slices: &[Axes]) -> Array<T> {
        self.check_slice_legitimacy(slices);

        todo!()
    }

    fn compute_slice_indicator_vector(&self, slices: &[Axes]) -> Vec<usize> {
        let mut slice_indicator_vector = Vec::<usize>::new();
        for (i, slice_indicator) in slices.iter().enumerate() {
            match slice_indicator {
                Axes::FullAxis => slice_indicator_vector.push(self.shape[i]),
                Axes::Range(m, n) => {
                    assert!(m <= n);
                    slice_indicator_vector.push(n - m + 1)
                }
                Axes::Slot(_) => slice_indicator_vector.push(0), // use `0` as placeholder for current axis with single slot
            }
        }
        slice_indicator_vector
    }

    fn compute_new_shape(&self, slices: &[Axes]) -> Vec<usize> {
        let slice_indicator_vector = self.compute_slice_indicator_vector(slices);
        slice_indicator_vector
            .into_iter()
            .filter(|&x| x != 0)
            .collect()
    }

    fn extract_data_from_slices(&self, slices: &[Axes]) -> Vec<T> {
        let new_shape = self.compute_new_shape(slices);
        let new_data = Vec::<T>::with_capacity(new_shape.iter().product());

        self.data
            .iter()
            .step_by(new_shape.into_iter().min().unwrap());
        todo!();
    }
}
