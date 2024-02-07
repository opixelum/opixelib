use std::iter::Sum;
use std::ops::{Add, Mul};

/// Sums two vectors of the same size, returning an error if sizes differ.
pub fn sum<T>(a: &[T], b: &[T]) -> Result<Vec<T>, &'static str>
where
    T: Add<Output = T> + Copy,
{
    if a.len() != b.len() {
        Err("Vectors must be of the same size")
    } else {
        Ok(a.iter().zip(b.iter()).map(|(a, b)| *a + *b).collect())
    }
}

/// Calculates the dot product of two vectors of the same size, returning an error if sizes differ.
pub fn dot<T>(a: &[T], b: &[T]) -> Result<T, &'static str>
where
    T: Mul<Output = T> + Add<Output = T> + Sum<T> + Copy,
{
    if a.len() != b.len() {
        Err("Vectors must be of the same size")
    } else {
        Ok(a.iter().zip(b.iter()).map(|(x, y)| *x * *y).sum())
    }
}
