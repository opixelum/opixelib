use num_traits::cast::NumCast;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
}

impl<T> Matrix<T>
where
    T: Default + Clone,
{
    pub fn new(shape: Vec<usize>) -> Matrix<T> {
        let total_size = shape.iter().product();
        Matrix {
            data: vec![T::default(); total_size],
            shape,
        }
    }

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

pub fn matrix_addition<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, &'static str>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Sum,
{
    if a.shape != b.shape {
        return Err("Dimension mismatch for matrix addition");
    }

    let mut result = Matrix::new(a.shape.clone());

    for (i, (a_elem, b_elem)) in a.data.iter().zip(b.data.iter()).enumerate() {
        result.data[i] = a_elem.clone() + b_elem.clone();
    }

    return Ok(result);
}

pub fn matrix_multiplication<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, &'static str>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Sum,
{
    if a.shape[1] != b.shape[0] {
        return Err("Dimension mismatch for matrix multiplication");
    }

    let mut result = Matrix::new(vec![a.shape[0], b.shape[1]]);

    for i in 0..a.shape[0] {
        for j in 0..b.shape[1] {
            let mut sum = T::default(); // Assuming T supports Default trait
            for k in 0..a.shape[1] {
                sum = sum + a.get(&[i, k]).unwrap().clone() * b.get(&[k, j]).unwrap().clone();
            }
            result.set(&[i, j], sum)?;
        }
    }

    Ok(result)
}

pub fn pairwise_operation<T, F>(a: &[T], b: &[T], mut operator: F) -> Matrix<T>
where
    T: Clone + Default,
    F: FnMut(&T, &T) -> T,
{
    let mut result = vec![vec![T::default(); b.len()]; a.len()];

    for (i, a_elem) in a.iter().enumerate() {
        for (j, b_elem) in b.iter().enumerate() {
            result[i][j] = operator(a_elem, b_elem);
        }
    }

    Matrix {
        data: result.into_iter().flatten().collect(),
        shape: vec![a.len(), b.len()],
    }
}

pub fn diagonal_operation<T, F>(matrix: &Matrix<T>, mut operator: F) -> Vec<T>
where
    T: Clone + Default,
    F: FnMut(T, T) -> T,
{
    let rows = matrix.shape[0];
    let cols = if rows > 0 { matrix.shape[1] } else { 0 };
    let mut result = vec![T::default(); rows + cols - 1];

    for i in 0..rows {
        for j in 0..cols {
            let result_index = i + j;
            if result_index < result.len() {
                result[result_index] = operator(
                    result[result_index].clone(),
                    matrix.get(&[i, j]).unwrap().clone(),
                );
            }
        }
    }

    result
}

pub fn convolution<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Clone + Default,
{
    let mut outer_product = pairwise_operation(&f, &g, |a, b| a.clone() * b.clone());
    diagonal_operation(&mut outer_product, |a, b| a + b)
}

pub fn average<T>(values: &[T]) -> T
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Clone + Default + Div<Output = T> + NumCast,
{
    let mut sum: T = T::default();
    for value in values {
        sum += value.clone();
    }
    sum / NumCast::from(values.len()).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_matrixes() {
        let matrix_1d: Matrix<u8> = Matrix::new(vec![3]);
        assert_eq!(matrix_1d.data, vec![0, 0, 0]);
        assert_eq!(matrix_1d.shape, vec![3]);

        let matrix_2d: Matrix<u8> = Matrix::new(vec![2, 3]);
        assert_eq!(matrix_2d.data, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(matrix_2d.shape, vec![2, 3]);

        let matrix_3d: Matrix<u8> = Matrix::new(vec![2, 2, 2]);
        assert_eq!(matrix_3d.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(matrix_3d.shape, vec![2, 2, 2]);
    }

    #[test]
    fn test_calculate_index_matrixes() {
        let matrix_1d = Matrix::<u8>::new(vec![3]);
        assert_eq!(matrix_1d.calculate_index(&[2]), Some(2));
        assert_eq!(matrix_1d.calculate_index(&[3]), None); // Out of bounds

        let matrix_2d = Matrix::<u8>::new(vec![2, 3]);
        assert_eq!(matrix_2d.calculate_index(&[1, 2]), Some(5));
        assert_eq!(matrix_2d.calculate_index(&[2, 2]), None); // Out of bounds

        let matrix_3d = Matrix::<u8>::new(vec![2, 2, 2]);
        assert_eq!(matrix_3d.calculate_index(&[1, 1, 1]), Some(7));
        assert_eq!(matrix_3d.calculate_index(&[2, 2, 2]), None); // Out of bounds
    }

    #[test]
    fn test_set_get_matrixes() {
        let mut matrix_1d = Matrix::<u8>::new(vec![3]);
        assert!(matrix_1d.set(&[1], 5).is_ok());
        assert_eq!(matrix_1d.get(&[1]), Some(&5));
        assert!(matrix_1d.set(&[3], 6).is_err()); // Out of bounds

        let mut matrix_2d = Matrix::<u8>::new(vec![2, 3]);
        assert!(matrix_2d.set(&[0, 2], 7).is_ok());
        assert_eq!(matrix_2d.get(&[0, 2]), Some(&7));
        assert!(matrix_2d.set(&[2, 2], 8).is_err()); // Out of bounds

        let mut matrix_3d = Matrix::<u8>::new(vec![2, 2, 2]);
        assert!(matrix_3d.set(&[1, 1, 1], 9).is_ok());
        assert_eq!(matrix_3d.get(&[1, 1, 1]), Some(&9));
        assert!(matrix_3d.set(&[2, 2, 2], 10).is_err()); // Out of bounds
    }

    #[test]
    fn test_convolution() {
        assert_eq!(convolution(&[1, 2, 3], &[4, 5, 6]), [4, 13, 28, 27, 18]);
        assert_eq!(convolution(&[1, 0, -1], &[2, 5]), [2, 5, -2, -5]);
        assert_eq!(
            convolution(&[0.0, 1.0, 0.5], &[1.0, -1.0]),
            [0.0, 1.0, -0.5, -0.5]
        );
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[1, 2, 3]), 2);
        assert_eq!(average(&[1.0, 1.0, 1.0]), 1.0);
        assert_eq!(average(&[0, 20]), 10);
        assert_eq!(average(&[-10.0, 10.0]), 0.0);
    }

    #[test]
    fn test_matrix_addition() {
        // Should work
        let a = Matrix {
            data: vec![1, 2, 3, 4, 5, 6],
            shape: vec![2, 3],
        };
        let b = Matrix {
            data: vec![7, 8, 9, 10, 11, 12],
            shape: vec![2, 3],
        };
        let result = matrix_addition(&a, &b).unwrap();
        assert_eq!(result.data, vec![8, 10, 12, 14, 16, 18]);
        assert_eq!(result.shape, vec![2, 3]);

        // Should fail due to dimension mismatch
        let c = Matrix {
            data: vec![1, 2, 3, 4],
            shape: vec![2, 2],
        };
        let result = matrix_addition(&a, &c);
        assert_eq!(result, Err("Dimension mismatch for matrix addition"));
    }

    #[test]
    fn test_matrix_multiplication() {
        // Should work
        let a = Matrix {
            data: vec![1, 2, 3, 4, 5, 6],
            shape: vec![2, 3],
        };
        let b = Matrix {
            data: vec![7, 8, 9, 10, 11, 12],
            shape: vec![3, 2],
        };
        let result = matrix_multiplication(&a, &b).unwrap();
        assert_eq!(result.data, vec![58, 64, 139, 154]);
        assert_eq!(result.shape, vec![2, 2]);

        // Should fail due to dimension mismatch
        let c = Matrix {
            data: vec![1, 2, 3, 4],
            shape: vec![2, 2],
        };
        let result = matrix_multiplication(&a, &c);
        assert_eq!(result, Err("Dimension mismatch for matrix multiplication"));
    }
}
