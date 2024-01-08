pub struct Tensor<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

impl<T> Tensor<T>
where
    T: Default + Clone,
{
    pub fn new(shape: Vec<usize>) -> Tensor<T> {
        let total_size = shape.iter().product();
        Tensor {
            data: vec![T::default(); total_size],
            shape,
        }
    }
}

impl<T> Tensor<T> {
    fn calculate_index(&self, indices: &[usize]) -> Option<usize> {
        if indices.len() != self.shape.len() {
            return None;
        }
        let mut index = 0;
        let mut multiplier = 1;
        for (i, &dim_index) in indices.iter().rev().enumerate() {
            if dim_index >= self.shape[self.shape.len() - 1 - i] {
                return None;
            }
            index += dim_index * multiplier;
            multiplier *= self.shape[self.shape.len() - 1 - i];
        }
        Some(index)
    }

    pub fn get(&self, indices: &[usize]) -> Option<&T> {
        let index = self.calculate_index(indices)?;
        self.data.get(index)
    }

    pub fn set(&mut self, indices: &[usize], value: T) -> Result<(), &'static str> {
        let index = self.calculate_index(indices).ok_or("Invalid indices")?;
        self.data[index] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tensors() {
        let tensor_1d: Tensor<u8> = Tensor::new(vec![3]);
        assert_eq!(tensor_1d.data, vec![0, 0, 0]);
        assert_eq!(tensor_1d.shape, vec![3]);

        let tensor_2d: Tensor<u8> = Tensor::new(vec![2, 3]);
        assert_eq!(tensor_2d.data, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(tensor_2d.shape, vec![2, 3]);

        let tensor_3d: Tensor<u8> = Tensor::new(vec![2, 2, 2]);
        assert_eq!(tensor_3d.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(tensor_3d.shape, vec![2, 2, 2]);
    }

    #[test]
    fn test_calculate_index_tensors() {
        let tensor_1d = Tensor::<u8>::new(vec![3]);
        assert_eq!(tensor_1d.calculate_index(&[2]), Some(2));
        assert_eq!(tensor_1d.calculate_index(&[3]), None); // Out of bounds

        let tensor_2d = Tensor::<u8>::new(vec![2, 3]);
        assert_eq!(tensor_2d.calculate_index(&[1, 2]), Some(5));
        assert_eq!(tensor_2d.calculate_index(&[2, 2]), None); // Out of bounds

        let tensor_3d = Tensor::<u8>::new(vec![2, 2, 2]);
        assert_eq!(tensor_3d.calculate_index(&[1, 1, 1]), Some(7));
        assert_eq!(tensor_3d.calculate_index(&[2, 2, 2]), None); // Out of bounds
    }

    #[test]
    fn test_set_get_tensors() {
        let mut tensor_1d = Tensor::<u8>::new(vec![3]);
        assert!(tensor_1d.set(&[1], 5).is_ok());
        assert_eq!(tensor_1d.get(&[1]), Some(&5));
        assert!(tensor_1d.set(&[3], 6).is_err()); // Out of bounds

        let mut tensor_2d = Tensor::<u8>::new(vec![2, 3]);
        assert!(tensor_2d.set(&[0, 2], 7).is_ok());
        assert_eq!(tensor_2d.get(&[0, 2]), Some(&7));
        assert!(tensor_2d.set(&[2, 2], 8).is_err()); // Out of bounds

        let mut tensor_3d = Tensor::<u8>::new(vec![2, 2, 2]);
        assert!(tensor_3d.set(&[1, 1, 1], 9).is_ok());
        assert_eq!(tensor_3d.get(&[1, 1, 1]), Some(&9));
        assert!(tensor_3d.set(&[2, 2, 2], 10).is_err()); // Out of bounds
    }
}
