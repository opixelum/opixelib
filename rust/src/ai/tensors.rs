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

    fn set(&mut self, index: usize, value: T) {
        self.data[index] = value;
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            self.data.get(index)
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

    fn set(&mut self, row: usize, column: usize, value: T) {
        let index = row * self.columns + column;
        self.data[index] = value;
    }

    fn get(&self, row: usize, column: usize) -> Option<&T> {
        if row < self.rows && column < self.columns {
            self.data.get(row * self.columns + column)
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

    fn set(&mut self, depth: usize, row: usize, column: usize, value: T) {
        let index = depth * self.rows * self.columns + row * self.columns + column;
        self.data[index] = value;
    }

    fn get(&self, depth: usize, row: usize, column: usize) -> Option<&T> {
        if depth < self.depth && row < self.rows && column < self.columns {
            self.data
                .get(depth * self.rows * self.columns + row * self.columns + column)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tensor1d() {
        let tensor: Tensor1D<u8> = Tensor1D::new(2);
        assert_eq!(tensor.data, vec![0, 0]);
        assert_eq!(tensor.length, 2);
    }

    #[test]
    fn test_new_tensor2d() {
        let tensor: Tensor2D<u8> = Tensor2D::new(2, 2);
        assert_eq!(tensor.data, vec![0, 0, 0, 0]);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);
    }

    #[test]
    fn test_new_tensor3d() {
        let tensor: Tensor3D<u8> = Tensor3D::new(2, 2, 2);
        assert_eq!(tensor.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(tensor.depth, 2);
        assert_eq!(tensor.rows, 2);
        assert_eq!(tensor.columns, 2);
    }
}
