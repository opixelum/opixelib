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
}
