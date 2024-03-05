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

pub fn sum<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, &'static str>
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

    Ok(result)
}

pub fn mul<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, &'static str>
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
    let outer_product = pairwise_operation(f, g, |a, b| a.clone() * b.clone());
    diagonal_operation(&outer_product, |a, b| a + b)
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
