use crate::array_general::Array;
use crate::scalar::*;
// use num_traits::*;
use std::fmt;

impl<T: Scalar> fmt::Display for Array<T> {
    fn fmt(&self, io: &mut fmt::Formatter) -> fmt::Result {
        let eltype = std::any::type_name::<T>();
        write!(io, "Array<{}, {:?}>:\n", eltype, self.shape)?; // print type info
        match self.shape.len() {
            1 => {
                let mut array_string = String::new();
                for v in self.data.iter() {
                    array_string += format!("{:^9.6}\n", v).as_str();
                }
                write!(io, "{}", array_string)?;
                Ok(())
            }
            2 => Array::display2d(&self, io),
            _ => {
                todo!()
            }
        }
    }
}

/* implementations for 2d Arrays */
impl<T: Scalar> Array<T> {
    /// display 2d Array
    pub fn display2d(&self, io: &mut fmt::Formatter) -> fmt::Result {
        assert!(self.shape.len() == 2);

        let row = self.shape[0];
        let col = self.shape[1];
        let mut array_string = String::new();

        let mut elem_string_length_max = 0;
        for value in self.data.iter() {
            let current_string_length = format!("{:>.6}", value).len() as i32 + 2;
            if current_string_length > elem_string_length_max {
                elem_string_length_max = current_string_length;
            }
        }
        // dbg!(elem_string_length_max);

        for i in 0..row {
            for j in 0..col {
                let data_index = self.calculate_data_index_from_array_indices([i, j].to_vec());
                array_string += format!(
                    "{:>width$.6}",
                    self.data[data_index],
                    width = elem_string_length_max as usize
                )
                .as_str();
            }
            array_string += "\n";
        }
        write!(io, "{}", array_string)?;
        Ok(())
    }
}
