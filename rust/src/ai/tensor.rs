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

    pub fn calculate_index(&self, indices: &[usize]) -> Option<usize> {
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
