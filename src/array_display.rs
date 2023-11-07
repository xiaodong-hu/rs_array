use crate::array_basic::Array;
use crate::scalar::*;
// use num_traits::*;
use std::fmt;

impl<T: Scalar> fmt::Display for Array<T> {
    fn fmt(&self, io: &mut fmt::Formatter) -> fmt::Result {
        use colored::*;
        let eltype = std::any::type_name::<T>();
        let array_info = format!("Array<{}, {:?}>:", eltype, self.shape).bold();
        write!(io, "{}", array_info)?; // print type info
        match self.shape.len() {
            1 => self.display1d(io),
            2 => self.display2d(io),
            _ => self.display_higher_dimensions(io),
        }
    }
}

/* implementations for 2d Arrays */
impl<T: Scalar> Array<T> {
    pub fn display1d(&self, io: &mut fmt::Formatter) -> fmt::Result {
        let mut array_string = String::new();
        for v in self.data.iter() {
            array_string += format!("\n{:^9.6}", v).as_str();
        }
        write!(io, "{}", array_string)?;
        Ok(())
    }

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
            array_string += "\n";
            for j in 0..col {
                let data_index = self.calculate_data_index_from_array_indices([i, j].to_vec());
                array_string += format!(
                    "{:>width$.6}",
                    self.data[data_index],
                    width = elem_string_length_max as usize
                )
                .as_str();
            }
        }
        write!(io, "{}", array_string)?;
        Ok(())
    }

    fn display_higher_dimensions(&self, io: &mut fmt::Formatter) -> fmt::Result {
        let num_of_2d_slices = self.shape.iter().skip(2).product::<usize>(); // skip the first two indices

        // Iterate over each 2D slice
        for slice_num in 0..num_of_2d_slices {
            // Compute the indices for the higher dimensions
            let mut higher_dim_indices = Vec::new();

            let mut remainder = slice_num;
            for &dim_size in self.shape.iter().skip(2) {
                higher_dim_indices.push(remainder % dim_size);
                remainder /= dim_size;
            }

            // Write the slice header
            let indices_str = higher_dim_indices
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(io, "\n[:, :, {}] = ", indices_str)?;

            // Display the 2D slice
            for row in 0..self.shape[0] {
                for col in 0..self.shape[1] {
                    let mut indices = vec![row, col];
                    indices.extend_from_slice(&higher_dim_indices);
                    let data_index = self.calculate_data_index_from_array_indices(indices);
                    write!(io, "{:>10.4} ", self.data[data_index])?;
                }
                writeln!(io)?;
            }
        }
        Ok(())
    }
}
