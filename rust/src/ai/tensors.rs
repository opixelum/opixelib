pub struct Tensor1D<T> {
    data: Vec<T>,
    length: usize,
}

impl<T> Tensor1D<T>
where
    T: Default + Clone,
{
    fn new(length: usize) -> Self {
        Tensor1D {
            data: vec![T::default(); length],
            length,
        }
    }

    fn set(&mut self, index: usize, value: T) -> Result<&mut Tensor1D<T>, &str> {
        if index < self.length {
            self.data[index] = value;
            Ok(self)
        } else {
            Err("ERROR: Tensor1D: set(): wrong index")
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        if index < self.length {
            self.data.get(index).cloned()
        } else {
            None
        }
    }
}

pub struct Tensor2D<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T> Tensor2D<T>
where
    T: Default + Clone,
{
    fn new(rows: usize, columns: usize) -> Self {
        Tensor2D {
            data: vec![T::default(); rows * columns],
            rows,
            columns,
        }
    }

    fn set(&mut self, row: usize, column: usize, value: T) -> Result<&mut Tensor2D<T>, &str> {
        if row < self.rows && column < self.columns {
            let index = row * self.columns + column;
            self.data[index] = value;
            Ok(self)
        } else {
            Err("ERROR: Tensor2D: set(): wrong index")
        }
    }

    fn get(&self, row: usize, column: usize) -> Option<T> {
        if row < self.rows && column < self.columns {
            self.data.get(row * self.columns + column).cloned()
        } else {
            None
        }
    }
}

pub struct Tensor3D<T> {
    data: Vec<T>,
    depth: usize,
    rows: usize,
    columns: usize,
}

impl<T> Tensor3D<T>
where
    T: Default + Clone,
{
    fn new(depth: usize, rows: usize, columns: usize) -> Self {
        Tensor3D {
            data: vec![T::default(); depth * rows * columns],
            depth,
            rows,
            columns,
        }
    }

    fn set(
        &mut self,
        depth: usize,
        row: usize,
        column: usize,
        value: T,
    ) -> Result<&mut Tensor3D<T>, &str> {
        if depth < self.depth && row < self.rows && column < self.columns {
            let index = depth * self.rows * self.columns + row * self.columns + column;
            self.data[index] = value;
            Ok(self)
        } else {
            Err("ERROR: Tensor3D: set(): wrong index")
        }
    }

    fn get(&self, depth: usize, row: usize, column: usize) -> Option<T> {
        if depth < self.depth && row < self.rows && column < self.columns {
            self.data
                .get(depth * self.rows * self.columns + row * self.columns + column)
                .cloned()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tensors() {
        let tensor: Tensor1D<u8> = Tensor1D::new(2);
        assert_eq!(tensor.data, vec![0, 0]);
        assert_eq!(tensor.length, 2);

        let tensor: Tensor2D<u8> = Tensor2D::new(2, 2);
        assert_eq!(tensor.data, vec![0, 0, 0, 0]);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);

        let tensor: Tensor3D<u8> = Tensor3D::new(2, 2, 2);
        assert_eq!(tensor.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(tensor.depth, 2);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);
    }

    #[test]
    fn test_set_tensors() {
        let mut tensor: Tensor1D<u8> = Tensor1D::new(2);
        assert_eq!(tensor.set(1, 1).unwrap().data, vec![0, 1]);
        assert!(tensor.set(2, 1).is_err());
        assert_eq!(tensor.data, vec![0, 1]);
        assert_eq!(tensor.length, 2);

        let mut tensor: Tensor2D<u8> = Tensor2D::new(2, 2);
        assert_eq!(tensor.set(1, 1, 1).unwrap().data, vec![0, 0, 0, 1]);
        assert!(tensor.set(2, 1, 1).is_err());
        assert_eq!(tensor.data, vec![0, 0, 0, 1]);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);

        let mut tensor: Tensor3D<u8> = Tensor3D::new(2, 2, 2);
        assert_eq!(
            tensor.set(1, 1, 1, 1).unwrap().data,
            vec![0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert!(tensor.set(2, 1, 1, 1).is_err());
        assert_eq!(tensor.data, vec![0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(tensor.depth, 2);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);
    }

    #[test]
    fn test_get_tensors() {
        let mut tensor: Tensor1D<u8> = Tensor1D::new(2);
        assert_eq!(tensor.set(1, 1).unwrap().get(1).unwrap(), 1);

        let mut tensor: Tensor2D<u8> = Tensor2D::new(2, 2);
        assert_eq!(tensor.set(1, 1, 1).unwrap().get(1, 1).unwrap(), 1);

        let mut tensor: Tensor3D<u8> = Tensor3D::new(2, 2, 2);
        assert_eq!(tensor.set(1, 1, 1, 1).unwrap().get(1, 1, 1).unwrap(), 1);
    }
}
